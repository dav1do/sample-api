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
}

// would normally come from a db crate but for simplicity putting here (probalby should have just done one crate)
#[derive(Debug, Default, Clone)]
pub struct MemoryDb {
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

    // maybe too much logic here but not enough time to refactor as I'd like
    #[tracing::instrument(level = "DEBUG", skip(self, token))]
    pub async fn add_favorite_city(&self, token: &str, city: City) -> Result<User, Error> {
        let mut authed = self.authed_users.lock().await;
        if let Some(mut user) = authed.get(token).map(|u| u.to_owned()) {
            // having two datasets to keep in sync is dumb. but for now we just use authed users as source of truth after signup
            user.favorite_cities.insert(city);
            authed.insert(token.to_string(), user.clone());
            Ok(user)
        } else {
            Err(Error::Unauthorized)
        }
    }

    // maybe too much logic here but not enough time to refactor as I'd like
    #[tracing::instrument(level = "DEBUG", skip(self, token))]
    pub async fn remove_favorite_city(&self, token: &str, city: &City) -> Result<User, Error> {
        let mut authed = self.authed_users.lock().await;
        if let Some(mut user) = authed.get(token).map(|u| u.to_owned()) {
            // having two datasets to keep in sync is dumb. but for now we just use authed users as source of truth after signup
            user.favorite_cities.remove(city);
            authed.insert(token.to_string(), user.clone());
            Ok(user)
        } else {
            Err(Error::Unauthorized)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
