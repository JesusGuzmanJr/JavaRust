use log::Level;
use std::collections::HashMap;

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Validation errors: `{0:?}`")]
    ValidationErrors(#[from] validator::ValidationErrors),

    #[error("Persistance error: `{0:?}`")]
    PersistanceError(#[from] sqlx::Error),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::ValidationErrors(validation_errors) => {
                let mut errors = HashMap::new();

                validation_errors
                    .field_errors()
                    .iter()
                    .for_each(|(&field, &validation_errors)| {
                        errors.entry(field).or_insert_with(Vec::new).append(
                            &mut validation_errors
                                .iter()
                                .filter_map(|e| e.message.as_deref())
                                .collect(),
                        )
                    });

                log::warn!("{:#?}", self);
                HttpResponse::BadRequest().json(errors)
            }
            Error::PersistanceError(sqlx_error) => {
                let (status, message, level) = match sqlx_error {
                    // https://www.postgresql.org/docs/current/errcodes-appendix.html
                    sqlx::Error::Database(error) => match error.code().as_deref() {
                        // unique violation
                        Some("23505") => (
                            StatusCode::CONFLICT,
                            error.message().to_owned(),
                            Level::Warn,
                        ),

                        // integrity constraint violation | foreign key violation
                        Some("23000" | "23503") => (
                            StatusCode::BAD_REQUEST,
                            error.message().to_owned(),
                            Level::Warn,
                        ),

                        _ => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            error.message().to_owned(),
                            Level::Error,
                        ),
                    },
                    error => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        error.to_string(),
                        Level::Error,
                    ),
                };

                log::log!(level, "{}", message);
                HttpResponse::build(status)
                    .set(ContentType::plaintext())
                    .body(message)
            }
        }
    }
}
