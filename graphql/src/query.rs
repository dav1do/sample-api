use async_graphql::Context;

use crate::GqlContext;

#[derive(Debug, Default)]
pub struct Query {}

#[async_graphql::Object]
impl Query {
    #[tracing::instrument(level = "INFO", skip(self, ctx))]
    async fn get_favorite_cities(&self, ctx: &Context<'_>) -> async_graphql::Result<bool> {
        let ctx = ctx.data::<GqlContext>().unwrap();
        todo!()
    }
}
