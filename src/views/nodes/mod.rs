mod getinfo;
mod listpeers;
use actix_web::web::{get, scope, ServiceConfig};

pub fn node_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/node")
            .route("getinfo/{node}", get().to(getinfo::getinfo))
            .route("listpeers/{node}", get().to(listpeers::listpeers)), // .route("listpeers", get().to(listpeers::listpeers)), // define view and URL
    );
}
