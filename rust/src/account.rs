use derive_more::{Display, From, Into};
use serde::Serialize;

use crate::datetime::Datetime;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: Id,
    pub created: Datetime,
    pub updated: Datetime,
    pub username: Username,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub status: Status,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Serialize, From, Into, Display)]
pub struct Id(pub uuid::Uuid);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, From, Into, Display)]
pub struct Username(pub String);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, From, Into, Display)]
pub struct Email(pub String);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, From, Into)]
pub struct PasswordHash(pub String);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, From, Display)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Unverified,
    Active,
    Disabled,
    Deleted,
}
