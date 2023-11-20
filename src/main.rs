use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use std::env;

mod json_serialization;
mod lnd;
mod storage;
mod views;

use crate::storage::cache::*;
use actix_web::web::Data;

#[macro_use]
extern crate log;

use log::{debug, error, info, log_enabled, Level};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _node_config_file = env::var("NODE_CONFIG_FILE").expect("NODE_CONFIG_FILE not set");

    let _redis_host_name =
        env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");

    let _redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();

    let redis_pool = create_pool().unwrap();

    // let mut redis_conn = create_connection(&redis_pool).await.unwrap();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    env_logger::init();

    // CombinedLogger::init(vec![TermLogger::new(
    //     LevelFilter::Info,
    //     Config::default(),
    //     TerminalMode::Mixed,
    //     ColorChoice::Auto,
    // )])
    // .unwrap();

    println!("server started successfully");

    HttpServer::new(move || {
        let redis_pool_data = Data::new(redis_pool.clone());
        let app = App::new()
            .app_data(redis_pool_data.clone())
            .configure(views::views_factory)
            .wrap(Logger::default());
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
