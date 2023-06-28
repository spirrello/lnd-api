// use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use std::env;
pub mod lnd;
pub mod views;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _node_config_file = env::var("NODE_CONFIG_FILE").expect("NODE_CONFIG_FILE not set");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
