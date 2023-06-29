mod describegraph;
mod getinfo;
mod listpeers;
mod walletbalance;
use actix_web::web::{get, scope, ServiceConfig};

pub fn node_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/node")
            .route("getinfo/{node}", get().to(getinfo::get_getinfo))
            .route("listpeers/{node}", get().to(listpeers::get_listpeers))
            .route(
                "walletbalance/{node}",
                get().to(walletbalance::walletbalance),
            )
            .route(
                "describegraph/{node}",
                get().to(describegraph::get_describegraph),
            ),
    );
}
