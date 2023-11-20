mod create;
use actix_web::web::{post, scope, ServiceConfig};

pub fn invoices_views_factory(app: &mut ServiceConfig) {
    app.service(scope("v1/invoices").route("create", post().to(create::create_invoice)));
}
