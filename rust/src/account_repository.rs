use sqlx::error::Error;

use crate::{
    account::Id,
    account_entity::{AccountEntity, Status},
    persistance,
};

/// Get the account by id.
pub async fn find_by_id(id: Id) -> Result<Option<AccountEntity>, Error> {
    sqlx::query_as!(
        AccountEntity,
        r#"SELECT id, created, updated, username, email, password_hash, status AS "status!: Status" FROM account WHERE id = $1"#,
        id.0
    )
    .fetch_optional(persistance::database_connector())
    .await
}
