use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Persistance error: `{0:?}`")]
    PersistanceError(#[from] sqlx::error::Error),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        log::warn!("{:?}", self);

        let status_code = match self {
            Error::PersistanceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status_code).body(self.to_string())
    }
}
