mod error;

use std::str::FromStr;

pub use crate::error::Error;

use actix_web::{web, HttpRequest, HttpResponse};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use graphql::{playground_source, GqlContext, GqlSchema, GraphQLPlaygroundConfig, MemoryDb};
use typed_headers::{Authorization, Header};

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

    pub fn into_graphql_ctx(&self, auth: Option<Authorization>) -> GqlContext {
        GqlContext {
            db: self.db.clone(),
            auth,
        }
    }
}

fn get_bearer_token(req: &HttpRequest) -> Option<Authorization> {
    req.headers()
        .get(typed_headers::Authorization::name())
        .and_then(|v| {
            // should log warnings in here
            let bearer = v.to_str().unwrap_or_default();
            if let Some(token) = typed_headers::Credentials::from_str(bearer).ok() {
                Some(Authorization(token))
            } else {
                None
            }
        })
}

#[tracing::instrument(skip(ctx, schema, request, gql_request), fields(request_id=%uuid::Uuid::new_v4().to_string()))]
pub async fn index(
    ctx: web::Data<AppData>,
    schema: web::Data<GqlSchema>,
    request: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let token = get_bearer_token(&request);
    let g_ctx = ctx.into_graphql_ctx(token);
    let req = gql_request.into_inner();
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
