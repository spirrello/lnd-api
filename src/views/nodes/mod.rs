mod getinfo;
mod listpeers;
mod walletbalance;
use actix_web::web::{get, scope, ServiceConfig};

use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

pub fn node_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/nodes")
            .route("{node}/getinfo", get().to(getinfo::get_getinfo))
            .route("{node}/listpeers", get().to(listpeers::get_listpeers))
            .route(
                "{node}/walletbalance",
                get().to(walletbalance::get_walletbalance),
            ),
    );
}

mod api1 {
    use actix_web::get;

    #[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Hello from api 1", body = String)
        )
    )]
    #[get("/api1/hello")]
    pub(super) async fn hello1() -> String {
        "hello from api 1".to_string()
    }
}

mod api2 {
    use actix_web::get;

    #[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Hello from api 2", body = String)
        )
    )]
    #[get("/api2/hello")]
    pub(super) async fn hello2() -> String {
        "hello from api 2".to_string()
    }
}
