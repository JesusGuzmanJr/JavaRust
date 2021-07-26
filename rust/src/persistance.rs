use once_cell::sync::OnceCell;

type Pool = sqlx::Pool<sqlx::Postgres>;

static POOL: OnceCell<Pool> = OnceCell::new();

pub async fn init() {
    let config = crate::config::app_config();
    let pool_size = config.postgres_connection_pool_size;

    let connection_timeout = std::time::Duration::new(config.postgres_connection_timeout_sec, 0);

    // panic if connection pool can't be created
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(pool_size)
        .connect_timeout(connection_timeout)
        .connect(&config.postgres_connection_uri)
        .await
        .expect("Couldn't create postgres connection pool.");

    pool.acquire().await.expect("Couldn't connect to postgres.");

    POOL.set(pool)
        .expect("Postgres connection pool is already initialized.");

    log::info!(
        "Started {} connection{} to Postgres.",
        pool_size,
        if pool_size == 1 { "" } else { "s" }
    );
}

/// Returns a reference to the database connection pool.
pub fn database_connector() -> &'static Pool {
    POOL.get()
        .expect("Postgres connection pool is not initialized.")
}
