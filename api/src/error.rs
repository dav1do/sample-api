use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Actix: {0:?}")]
    Actix(#[from] actix_web::Error),
}
