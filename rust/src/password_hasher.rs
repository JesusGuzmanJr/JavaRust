use argon2::{password_hash::Error, Algorithm, Argon2, Params, Version};
use once_cell::sync::OnceCell;
use rand_core::RngCore;

use crate::account::{PasswordHash, PasswordSalt};

const ALGORITHM: Algorithm = Algorithm::Argon2id;
const VERSION: Version = Version::V0x13;
const TIME_COST: u32 = 5;
const MEMORY_COST: u32 = 4096;
const PARALLELISM: u32 = 2;
const SALT_LENGTH: usize = 16;
const HASH_LENGTH: usize = 32;

static ARGON2: OnceCell<Argon2> = OnceCell::new();

pub fn init() {
    let params = Params::new(MEMORY_COST, TIME_COST, PARALLELISM, None).expect("argon2 init error");
    ARGON2
        .set(Argon2::new(ALGORITHM, VERSION, params))
        .map_err(|_| ())
        .expect("already initialized");
}

pub fn create_salt() -> PasswordSalt {
    let mut salt = [0; SALT_LENGTH];
    rand_core::OsRng.fill_bytes(&mut salt);
    Vec::from(salt).into()
}

pub fn hash(password: &str, password_salt: &PasswordSalt) -> Result<PasswordHash, Error> {
    let mut hash = [0; HASH_LENGTH];

    ARGON2.get().expect("not initialized").hash_password_into(
        password.as_ref(),
        password_salt.0.as_ref(),
        &mut hash,
    )?;
    Ok(Vec::from(hash).into())
}

pub fn verify(
    password: &str,
    password_hash: &PasswordHash,
    password_salt: &PasswordSalt,
) -> Result<bool, Error> {
    let test_hash = hash(password, password_salt)?;
    Ok(&test_hash == password_hash)
}
