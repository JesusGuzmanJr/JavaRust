use uuid::Uuid;

use crate::datetime::Datetime;

pub struct AccountEntity {
    pub id: Uuid,
    pub created: Datetime,
    pub updated: Datetime,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub status: Status,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "ACCOUNT_STATUS")]
pub enum Status {
    Unverified,
    Active,
    Disabled,
    Deleted,
}
