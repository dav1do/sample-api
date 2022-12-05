mod error;
mod mutation;
mod query;
mod types;

use std::{collections::HashMap, sync::Arc};

use async_graphql::EmptySubscription;
use mutation::Mutation;
use query::Query;

pub use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
pub use error::Error;
use tokio::sync::Mutex;
use typed_headers::Authorization;
pub use types::*;
pub type GqlSchema = Schema<Query, Mutation, EmptySubscription>;

// would normally return SchemaBuilder to allow disabling introspection or other config based on api env
pub fn new_schema() -> GqlSchema {
    Schema::build(
        Query::default(),
        Mutation::default(),
        EmptySubscription::default(),
    )
    .finish()
}

#[derive(Debug, Default, Clone)]
pub struct GqlContext {
    pub db: MemoryDb,
    pub auth: Option<Authorization>,
}

impl GqlContext {
    pub async fn verify_token(&self) -> Result<User, Error> {
        // normally this would use use a JWT and we'd have a Box<dyn JwtAuthorizer> or something on Self we could instantiate from the JWKs
        if let Some(t) = self.auth.as_ref().map(|t| t.as_bearer()).flatten() {
            let authed = self
                .db
                .get_authed_user(t.as_str())
                .await
                .ok_or_else(|| Error::Unauthorized)?;

            Ok(self.db.get_signed_up_user(&authed.email).await.unwrap())
        } else {
            Err(Error::Unauthorized)
        }
    }
}

// would normally come from a db crate but for simplicity putting here (probalby should have just done one crate)
#[derive(Debug, Default, Clone)]
pub struct MemoryDb {
    // could do Arc<RwLock<HashMap<String, Mutex<User>>>> or use dashmap for better concurrency
    users: Arc<Mutex<HashMap<String, User>>>, // Email -> User
    /// would normally have a JWT and validate & check sub or something but for simplicity
    authed_users: Arc<Mutex<HashMap<String, User>>>, // token -> User
}

impl MemoryDb {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::default())),
            authed_users: Arc::new(Mutex::new(HashMap::default())),
        }
    }

    // for now we're just overwriting and not doing upserts
    /// Returns (token, user)
    #[tracing::instrument(level = "DEBUG", skip(self))]
    pub async fn add_user(&self, user: User) -> Result<User, Error> {
        let mut users = self.users.lock().await;
        users.insert(user.email.clone(), user.clone());
        Ok(user)
    }

    /// returns the user token (should be a JWT in practice)
    #[tracing::instrument(level = "DEBUG", skip(self))]
    pub async fn login_user(&self, user: User) -> Result<String, Error> {
        let mut authed = self.authed_users.lock().await;
        let token = uuid::Uuid::new_v4().to_string(); //should be a JWT
        authed.insert(token.clone(), user);
        Ok(token)
    }

    #[tracing::instrument(level = "DEBUG", skip(self,))]
    pub async fn get_signed_up_user(&self, email: &str) -> Option<User> {
        let users = self.users.lock().await;
        users.get(email).map(|u| u.to_owned())
    }

    #[tracing::instrument(level = "DEBUG", skip(self, token))]
    pub async fn get_authed_user(&self, token: &str) -> Option<User> {
        let authed = self.authed_users.lock().await;
        authed.get(token).map(|u| u.to_owned())
    }

    /// Assumes the user has been authenticated already
    #[tracing::instrument(level = "DEBUG", skip(self, user))]
    pub async fn add_favorite_city(&self, user: &User, city: City) -> Result<User, Error> {
        let mut users = self.users.lock().await;
        if let Some(updated) = users.get_mut(&user.email) {
            updated.favorite_cities.insert(city);
            Ok(updated.to_owned())
        } else {
            tracing::error!(
                ?user,
                "missing user even though authorized when adding favorite city!"
            );
            return Err(Error::Unauthorized);
        }
    }

    /// Assumes the user has been authenticated already
    #[tracing::instrument(level = "DEBUG", skip(self, user))]
    pub async fn remove_favorite_city(&self, user: &User, city: &City) -> Result<User, Error> {
        let mut users = self.users.lock().await;
        if let Some(updated) = users.get_mut(&user.email) {
            updated.favorite_cities.remove(city);
            Ok(updated.to_owned())
        } else {
            tracing::error!(
                ?user,
                "missing user even though authorized when removing favorite city!"
            );
            return Err(Error::Unauthorized);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
