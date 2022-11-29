mod error;

pub use crate::error::Error;
use actix_web::{web, HttpResponse};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use graphql::{playground_source, GqlSchema, GraphQLPlaygroundConfig};

#[tracing::instrument(skip(schema, request), fields(request_id=%uuid::Uuid::new_v4().to_string()))]
pub async fn index(
    // ctx: web::Data<Context>,
    schema: web::Data<GqlSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
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
