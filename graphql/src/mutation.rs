#[derive(Debug, Default)]
pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn signup(&self, input: bool) -> async_graphql::Result<bool> {
        todo!()
    }

    async fn login(&self, input: bool) -> async_graphql::Result<bool> {
        todo!()
    }

    async fn add_favorite_city(&self, input: bool) -> async_graphql::Result<bool> {
        todo!()
    }
}
