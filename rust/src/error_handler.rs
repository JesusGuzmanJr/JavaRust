use std::collections::HashMap;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Persistance error: `{0:?}`")]
    PersistanceError(#[from] sqlx::error::Error),

    #[error("Validation error: `{0:?}`")]
    ValidationErrors(#[from] validator::ValidationErrors),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        log::warn!("{:?}", self);

        let status_code = match self {
            Error::PersistanceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ValidationErrors(_) => StatusCode::BAD_REQUEST,
        };

        if let Error::ValidationErrors(validation_errors) = self {
            let mut errors = HashMap::new();

            validation_errors
                .field_errors()
                .iter()
                .for_each(|(&field, &validation_errors)| {
                    // collect the messages from all the validation errors
                    let mut error_messages = validation_errors
                        .iter()
                        .filter_map(|e| e.message.clone())
                        .collect::<Vec<_>>();

                    // add the messages to the field
                    errors
                        .entry(field)
                        .or_insert_with(Vec::new)
                        .append(&mut error_messages)
                });

            HttpResponse::build(status_code).json(errors)
        } else {
            HttpResponse::build(status_code).body(self.to_string())
        }
    }
}
