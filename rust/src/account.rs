use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};

use crate::datetime::Datetime;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct Account {
    pub id: Id,
    pub created: Datetime,
    pub updated: Datetime,
    pub username: Username,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub status: Status,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Serialize, Deserialize, From, Into, Display)]
pub struct Id(pub uuid::Uuid);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, From, Into, Display)]
pub struct Username(pub String);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, From, Into, Display)]
pub struct Email(pub String);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, From, Into)]
pub struct PasswordHash(pub String);

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, From, Display)]
pub enum Status {
    Unverified,
    Active,
    Disabled,
    Deleted,
}
