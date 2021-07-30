use sqlx::{error::Error, postgres::PgQueryResult};

use crate::{
    account::Id,
    account_entity::{AccountEntity, Status},
    persistance,
};

pub async fn find_by_id(id: Id) -> Result<Option<AccountEntity>, Error> {
    sqlx::query_as!(
        AccountEntity,
        r#"SELECT id, created, updated, username, email, password_hash, password_salt, status AS "status!: Status" FROM account WHERE id = $1"#,
        id.0
    )
    .fetch_optional(persistance::database_connector())
    .await
}

pub async fn save_and_flush(account_entity: &AccountEntity) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        r#"INSERT INTO account VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
        account_entity.id,
        account_entity.created,
        account_entity.updated,
        account_entity.username,
        account_entity.email,
        account_entity.password_hash,
        account_entity.password_salt,
        &account_entity.status as &Status,
    )
    .execute(persistance::database_connector())
    .await
}
