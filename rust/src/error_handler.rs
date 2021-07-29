use std::collections::HashMap;

use actix_web::{HttpResponse, ResponseError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Persistance error: `{0:?}`")]
    PersistanceError(#[from] sqlx::error::Error),

    #[error("Validation errors: `{0:?}`")]
    ValidationErrors(#[from] validator::ValidationErrors),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::PersistanceError(_) => {
                log::error!("{:#?}", self);
                HttpResponse::InternalServerError().body(self.to_string())
            }
            Error::ValidationErrors(validation_errors) => {
                let mut errors = HashMap::new();

                validation_errors
                    .field_errors()
                    .iter()
                    .for_each(|(&field, &validation_errors)| {
                        errors.entry(field).or_insert_with(Vec::new).append(
                            &mut validation_errors
                                .iter()
                                .filter_map(|e| e.message.clone())
                                .map(|m| m.into_owned())
                                .collect(),
                        )
                    });
                log::warn!("{:#?}", self);
                HttpResponse::BadRequest().json(errors)
            }
        }
    }
}
