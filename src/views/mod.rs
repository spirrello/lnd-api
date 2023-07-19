mod graph;
mod invoices;
mod nodes;
use actix_web::web::ServiceConfig;

use self::graph::graph_views_factory;
use self::invoices::invoices_views_factory;
use self::nodes::node_views_factory;

pub fn views_factory(app: &mut ServiceConfig) {
    node_views_factory(app);
    graph_views_factory(app);
    invoices_views_factory(app);
}
