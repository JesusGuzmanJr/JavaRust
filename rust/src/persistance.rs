use std::time::Duration;

use once_cell::sync::OnceCell;

type Pool = sqlx::Pool<sqlx::Postgres>;

static POOL: OnceCell<Pool> = OnceCell::new();

pub async fn init() {
    let config = crate::config::app_config();

    let connection_timeout = Duration::new(config.postgres_timeout_sec, 0);
    let max_lifetime = Duration::new(config.postgres_max_life_time_minutes * 60, 0);
    let postgres_uri = std::env::var(&config.postgres_uri_env_var_name).unwrap_or_else(|_| {
        panic!(
            "Could not find an environmental variable {}",
            &config.postgres_uri_env_var_name
        )
    });

    // panic if connection pool can't be created
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.postgres_max_pool_size)
        .min_connections(config.postgres_min_pool_size)
        .connect_timeout(connection_timeout)
        .max_lifetime(max_lifetime)
        .connect(&postgres_uri)
        .await
        .expect("Couldn't create postgres connection pool.");

    pool.acquire().await.expect("Couldn't connect to postgres.");

    POOL.set(pool)
        .expect("Postgres connection pool is already initialized.");
}

/// Returns a reference to the database connection pool.
pub fn database_connector() -> &'static Pool {
    POOL.get()
        .expect("Postgres connection pool is not initialized.")
}
