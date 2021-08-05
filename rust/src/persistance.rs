use once_cell::sync::OnceCell;

type Pool = sqlx::Pool<sqlx::Postgres>;

static POOL: OnceCell<Pool> = OnceCell::new();

pub async fn init() {
    let config = crate::config::app_config();

    let connection_timeout = std::time::Duration::new(config.postgres_timeout_sec, 0);
    let max_lifetime = std::time::Duration::new(config.postgres_max_life_time_minutes * 60, 0);

    // panic if connection pool can't be created
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.postgres_max_pool_size)
        .min_connections(config.postgres_min_pool_size)
        .connect_timeout(connection_timeout)
        .max_lifetime(max_lifetime)
        .connect(&config.postgres_uri)
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
