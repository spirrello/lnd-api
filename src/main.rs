use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use std::env;

mod json_serialization;
mod lnd;
mod storage;
mod views;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _node_config_file = env::var("NODE_CONFIG_FILE").expect("NODE_CONFIG_FILE not set");

    let _redis_host_name =
        env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");

    let _redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(|| {
        let app = App::new()
            .configure(views::views_factory)
            .wrap(Logger::default());
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
