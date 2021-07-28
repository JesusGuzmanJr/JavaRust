use actix_web::{
    get, post,
    web::{Json, Path},
    HttpResponse,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    account::Account, account_repository, create_account::CreateAccount, error_handler::Error,
};

#[get("/account/{id}")]
async fn get_account(Path(id): Path<Uuid>) -> Result<HttpResponse, Error> {
    let account = Account {
        id: uuid::Uuid::new_v4().into(),
        created: chrono::Utc::now(),
        updated: chrono::Utc::now(),
        username: "jesus".to_string().into(),
        email: "email".to_string().into(),
        password_hash: "hash".to_string().into(),
        status: crate::account::Status::Active,
    };
    Ok(HttpResponse::Ok().json(account))
    // let response = match account_repository::find_by_id(id.0.into()).await? {
    //     None => HttpResponse::NotFound().finish(),
    //     Some(account) => HttpResponse::Ok().json::<Account>(account.into()),
    // };
    // Ok(response)
}

#[post("/account")]
async fn create_account(Json(create_account): Json<CreateAccount>) -> Result<HttpResponse, Error> {
    create_account.validate()?;
    Ok(HttpResponse::NotImplemented().finish())
}
