use async_graphql::{Context, ErrorExtensions};

use crate::{City, GqlContext, User};

#[derive(Debug, Default)]
pub struct Query {}

#[async_graphql::Object]
impl Query {
    #[tracing::instrument(level = "INFO", skip(self, ctx, _input))]
    /// Gets user data including favorite cities
    async fn get_user(
        &self,
        ctx: &Context<'_>,
        // not needed as we have a token now
        _input: GetUserInput,
    ) -> async_graphql::Result<UserData> {
        let ctx = ctx.data::<GqlContext>().unwrap();
        ctx.verify_token()
            .await
            .map(UserData::from)
            .map_err(|e| e.extend())
    }
}

#[derive(Clone, Debug, async_graphql::InputObject)]
pub struct GetUserInput {
    // this should really be a auth header but starting simple
    token: String,
}

#[derive(Clone, Debug, async_graphql::SimpleObject)]
pub struct UserData {
    email: String,
    name: String,
    favorite_cities: Vec<CityData>,
}

#[derive(Clone, Debug, async_graphql::SimpleObject)]
pub struct CityData {
    name: String,
    country: String,
}

impl From<User> for UserData {
    fn from(v: User) -> Self {
        let favorite_cities = v
            .favorite_cities
            .into_iter()
            .map(CityData::from)
            .collect::<_>();
        Self {
            email: v.email,
            name: v.name,
            favorite_cities,
        }
    }
}

impl From<City> for CityData {
    fn from(v: City) -> Self {
        Self {
            name: v.name,
            country: v.country,
        }
    }
}
