mod nodes;

use actix_web::web::ServiceConfig;

use self::nodes::node_views_factory;

pub fn views_factory(app: &mut ServiceConfig) {
    node_views_factory(app);
}
