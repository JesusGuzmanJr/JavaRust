use crate::{
    account::{Account, Status},
    account_entity::{AccountEntity, Status as StatusEntity},
};

impl From<AccountEntity> for Account {
    fn from(account_entity: AccountEntity) -> Account {
        Account {
            id: account_entity.id.into(),
            created: account_entity.created,
            updated: account_entity.updated,
            username: account_entity.username.into(),
            email: account_entity.email.into(),
            password_hash: account_entity.password_hash.into(),
            status: account_entity.status.into(),
        }
    }
}

impl From<StatusEntity> for Status {
    fn from(status: StatusEntity) -> Status {
        match status {
            StatusEntity::Unverified => Status::Unverified,
            StatusEntity::Active => Status::Active,
            StatusEntity::Disabled => Status::Disabled,
            StatusEntity::Deleted => Status::Deleted,
        }
    }
}
