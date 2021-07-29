use actix_web::{
    get, post,
    web::{Json, Path},
    HttpResponse,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    account::Account, account_entity, account_entity::AccountEntity, account_repository,
    create_account::CreateAccount, error_handler::Error,
};

#[get("/account/{id}")]
async fn get_account(Path(id): Path<Uuid>) -> Result<HttpResponse, Error> {
    let response = match account_repository::find_by_id(id.into()).await? {
        Some(account) => HttpResponse::Ok().json::<Account>(account.into()),
        None => HttpResponse::NotFound().finish(),
    };
    Ok(response)
}

#[post("/account")]
async fn create_account(Json(create_account): Json<CreateAccount>) -> Result<HttpResponse, Error> {
    create_account.validate()?;

    let now = chrono::Utc::now();

    let account_entity = AccountEntity {
        id: Uuid::new_v4(),
        created: now,
        updated: now,
        username: create_account.username.into(),
        email: create_account.email.into(),
        password_hash: String::new().into(),
        status: account_entity::Status::Unverified,
    };

    account_repository::save_and_flush(&account_entity).await?;
    Ok(HttpResponse::Ok().json::<Account>(account_entity.into()))
}
