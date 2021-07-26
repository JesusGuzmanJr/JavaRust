use actix_web::{get, web::Path, HttpResponse};
use uuid::Uuid;

use crate::{account::Account, account_repository, error::Error};

#[get("/account/{id}")]
async fn get_account(id: Path<Uuid>) -> Result<HttpResponse, Error> {
    let response = match account_repository::find_by_id(id.0.into()).await? {
        None => HttpResponse::NotFound().finish(),
        Some(account) => HttpResponse::Ok().json::<Account>(account.into()),
    };
    Ok(response)
}
