use actix_web::{App, HttpServer};

mod info_endpoint;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            // <response status code> for <path> <remote/proxy ip address> in <seconds>s
            .wrap(actix_web::middleware::Logger::new("%s for %U %a in %Ts"))
            .service(info_endpoint::get_info)
    })
    .bind("localhost:9090")?
    .run()
    .await
}
