mod error;

pub use crate::error::Error;

use actix_web::{web, HttpResponse};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use graphql::{playground_source, GqlContext, GqlSchema, GraphQLPlaygroundConfig, MemoryDb};

#[derive(Clone, Debug)]
pub struct AppData {
    pub db: MemoryDb, // normally would use a db pool/trait to mock this out
}

impl AppData {
    pub fn new() -> Self {
        Self {
            db: MemoryDb::new(),
        }
    }

    pub fn into_graphql_ctx(&self) -> GqlContext {
        GqlContext {
            db: self.db.clone(),
        }
    }
}

#[tracing::instrument(skip(ctx, schema, request), fields(request_id=%uuid::Uuid::new_v4().to_string()))]
pub async fn index(
    ctx: web::Data<AppData>,
    schema: web::Data<GqlSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let g_ctx = ctx.into_graphql_ctx();
    let req = request.into_inner();
    let req = req.data(g_ctx);
    schema.execute(req).await.into()
}

#[tracing::instrument(fields(request_id=%uuid::Uuid::new_v4().to_string()))]
pub async fn playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql")
                .subscription_endpoint("/graphql/subscriptions"),
        ))
}
