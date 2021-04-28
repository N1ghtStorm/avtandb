use std::sync::Arc;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[path = "./datamodel/core_model.rs"]
mod core_model;

#[path = "./datamodel/common_model.rs"]
mod common_model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut graph_collection = initialize_graph_collection();

    HttpServer::new(|| {
        App::new()
            //.register_controllers()
            //.service(cat_controller::get_cats)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/// initialize common graph collection for all programm lifetime
fn initialize_graph_collection() -> Arc<common_model::GraphCollectionFacade> {
    Arc::new(common_model::GraphCollectionFacade {graph_collection: Arc::new(Vec::new())})
}