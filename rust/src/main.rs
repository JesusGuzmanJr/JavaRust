use actix_web::{App, HttpServer};

mod account;
mod account_endpoint;
mod account_entity;
mod account_mapper;
mod account_repository;
mod config;
mod create_account;
mod datetime;
mod error_handler;
mod info_endpoint;
mod password_hasher;
mod persistance;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    color_backtrace::install();
    let config = config::app_config();

    // initialize logging
    let mut log_builder = env_logger::Builder::new();
    log_builder.parse_filters(&config.logging_directive);
    log_builder.init();

    // initialize database connection
    persistance::init().await;

    // init password hasher
    password_hasher::init();

    // initialize server
    HttpServer::new(|| {
        App::new()
            // <response status code> for <path> <remote/proxy ip address> in <seconds>s
            .wrap(actix_web::middleware::Logger::new("%s for %U %a in %Ts"))
            .service(info_endpoint::get_info)
            .service(account_endpoint::get_account)
            .service(account_endpoint::create_account)
    })
    .bind(format!("{}:{}", config.address, config.port))?
    .run()
    .await
}
