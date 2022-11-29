mod mutation;
mod query;

use async_graphql::EmptySubscription;
use mutation::Mutation;
use query::Query;

pub use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
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
pub struct GqlContext {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
