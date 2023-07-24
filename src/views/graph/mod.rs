mod describegraph;
mod getchaninfo;
mod getnodemetrics;
use actix_web::web::{get, scope, ServiceConfig};

pub fn graph_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/graph")
            .route(
                "{node}/describegraph",
                get().to(describegraph::get_describegraph),
            )
            .route(
                "{node}/getchaninfo/{chan_id}",
                get().to(getchaninfo::get_getchaninfo),
            )
            .route(
                "{node}/getnodemetrics",
                get().to(getnodemetrics::get_getnodemetrics),
            ),
    );
}
