use actix_web::{
    get, post,
    web::{Json, Path},
    HttpResponse,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    account::Account,
    account_entity::{AccountEntity, Status::Unverified},
    account_repository,
    create_account::CreateAccount,
    error_handler::Error,
    password_hasher,
};

#[get("/account/{id}")]
async fn get_account(Path(id): Path<Uuid>) -> Result<HttpResponse, Error> {
    Ok(match account_repository::find_by_id(id.into()).await? {
        Some(account) => HttpResponse::Ok().json::<Account>(account.into()),
        None => HttpResponse::NotFound().finish(),
    })
}

#[post("/account")]
async fn create_account(Json(create_account): Json<CreateAccount>) -> Result<HttpResponse, Error> {
    create_account.validate()?;

    let now = chrono::Utc::now();

    let password_salt = password_hasher::create_salt();
    let password_hash = password_hasher::hash(&create_account.password, &password_salt)?;

    let account_entity = AccountEntity {
        id: Uuid::new_v4(),
        created: now,
        updated: now,
        username: create_account.username,
        email: create_account.email,
        password_hash: password_hash.into(),
        password_salt: password_salt.into(),
        status: Unverified,
    };

    account_repository::save_and_flush(&account_entity).await?;
    Ok(HttpResponse::Ok().json::<Account>(account_entity.into()))
}
