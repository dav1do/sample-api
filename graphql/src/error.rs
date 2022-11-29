use async_graphql::ErrorExtensions;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("{0}")]
    Custom(String),
}

impl ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            Error::Unauthorized => {
                tracing::warn!("Unauthorized");
                e.set("code", "UNAUTHORIZED");
            }
            error => {
                tracing::error!(?error, "Error");
                e.set("code", "INTERNAL_SERVER_ERROR");
            }
        })
    }
}
