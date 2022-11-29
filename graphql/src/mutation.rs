use std::collections::HashMap;

use async_graphql::{Context, ErrorExtensions};

use crate::{query::CityData, City, Error, GqlContext, User};

#[derive(Debug, Default)]
pub struct Mutation;

// while it's tedious, prefer to use dedicated input/output objects to make the graph more extensible in the future

#[async_graphql::Object]
impl Mutation {
    async fn signup(
        &self,
        ctx: &Context<'_>,
        input: SignupInput,
    ) -> async_graphql::Result<SignupResult> {
        // normally, I would wrap the logic in these handlers into a service struct or something
        // this would allow writing tests and calling the service code without needing the graphql or server level

        // unwrapping all of these for expediency but if this ever happens something is very very wrong
        let g_ctx = ctx.data::<GqlContext>().unwrap();
        match g_ctx.db.add_user(input.into()).await {
            Ok(u) => Ok(u.into()),
            Err(e) => Err(Error::Custom(format!("{}", e)).extend()),
        }
    }

    /// Returns token to use for adding/removing favorite cities
    async fn login(
        &self,
        ctx: &Context<'_>,
        input: LoginInput,
    ) -> async_graphql::Result<LoginResult> {
        let g_ctx = ctx.data::<GqlContext>().unwrap();

        if let Some(user) = g_ctx.db.get_signed_up_user(&input.email).await {
            if user.verify_password(&input.password) {
                let token = g_ctx.db.login_user(user).await?;
                Ok(LoginResult { token })
            } else {
                // probably don't want to say this, but for now it works
                Err(Error::Custom("Invalid credentials".into()).extend())
            }
        } else {
            Err(Error::Custom(format!("User '{}' has not signed up", input.email)).extend())
        }
    }

    async fn add_favorite_city(
        &self,
        ctx: &Context<'_>,
        input: AddFavoriteCityInput,
    ) -> async_graphql::Result<AddFavoriteCityResult> {
        let g_ctx = ctx.data::<GqlContext>().unwrap();

        let city = City {
            name: input.name,
            country: input.country,
        };

        // too much logic in DB here, should validate auth at the service or api level
        let _user = g_ctx
            .db
            .add_favorite_city(&input.token, city.clone())
            .await
            .map_err(|e| e.extend())?;

        Ok(AddFavoriteCityResult { city: city.into() })
    }

    async fn remove_favorite_city(
        &self,
        ctx: &Context<'_>,
        input: AddFavoriteCityInput,
    ) -> async_graphql::Result<RemoveFavoriteCityResult> {
        let g_ctx = ctx.data::<GqlContext>().unwrap();

        let _user = g_ctx
            .db
            .remove_favorite_city(&input.token, &input.name)
            .await
            .map_err(|e| e.extend())?;

        Ok(RemoveFavoriteCityResult { success: true })
    }
}

#[derive(Clone, Debug, async_graphql::InputObject)]
pub struct SignupInput {
    email: String,
    name: String,
    password: String,
}
// normally woudld do a union of SuccessObject | ErrorObject to support returning "errors as data"
#[derive(Clone, Debug, async_graphql::SimpleObject)]
pub struct SignupResult {
    email: String,
    name: String,
}

#[derive(Clone, Debug, async_graphql::InputObject)]
pub struct LoginInput {
    email: String,
    password: String,
}

#[derive(Clone, Debug, async_graphql::InputObject)]
pub struct AddFavoriteCityInput {
    // again, should be jwt not input
    token: String,
    // could collapse into CreateCityInput or something to use in other routes
    name: String,
    country: String,
}

#[derive(Clone, Debug, async_graphql::SimpleObject)]
pub struct AddFavoriteCityResult {
    city: CityData,
}

#[derive(Clone, Debug, async_graphql::InputObject)]
pub struct RemoveFavoriteCityInput {
    // again, should be jwt not input
    token: String,
    // could collapse into CreateCityInput or something to use in other routes
    name: String,
}

#[derive(Clone, Debug, async_graphql::SimpleObject)]
pub struct RemoveFavoriteCityResult {
    success: bool,
}

#[derive(Clone, Debug, async_graphql::SimpleObject)]
pub struct LoginResult {
    token: String,
}

impl From<SignupInput> for User {
    fn from(v: SignupInput) -> Self {
        Self {
            name: v.name,
            email: v.email,
            password: v.password,
            favorite_cities: HashMap::default(),
        }
    }
}

impl From<User> for SignupResult {
    fn from(v: User) -> Self {
        Self {
            name: v.name,
            email: v.email,
        }
    }
}
