use dashmap::DashSet;
use goose::prelude::*;
use once_cell::sync::OnceCell;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const PASSWORD_LENGTH: usize = 16;
const USERNAME_LENGTH: usize = 16;
const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

type Map = Arc<DashSet<String>>;

static ACCOUNT_IDS: OnceCell<Map> = OnceCell::new();

fn account_ids() -> Map {
    Arc::clone(ACCOUNT_IDS.get_or_init(|| Arc::new(DashSet::new())))
}

fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_taskset(
            taskset!("Create Accounts").register_task(task!(create_account).set_weight(10)?),
        )
        .execute()?;
    Ok(())
}

async fn create_account(user: &GooseUser) -> GooseTaskResult {
    let account = CreateAccount::generate_new();
    let request_builder = user.goose_post("/account").await?.json(&account);
    let goose_response = user.goose_send(request_builder, None).await?;
    let account = goose_response.response?.json::<Account>().await?;
    account_ids().insert(account.id);
    Ok(())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccount {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl CreateAccount {
    fn generate_new() -> Self {
        let username = (0..USERNAME_LENGTH)
            .map(|_| {
                let idx = rand::thread_rng().gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        let password = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(PASSWORD_LENGTH)
            .map(char::from)
            .collect();
        CreateAccount {
            email: format!("{}@email.com", &username),
            username,
            password,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Account {
    id: String,
    // other fields not deserialized
}
