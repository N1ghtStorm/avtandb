use std::sync::Arc;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

#[path = "./datamodel/core_model.rs"]
mod core_model;

#[path = "./datamodel/common_model.rs"]
mod common_model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(initialize_graph_collection());

    HttpServer::new( move || {
        App::new()
            .app_data(data.clone())
            .service(hello)
            .service(hi)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/create_graph")]
async fn hello(arc_graph_col_fac: web::Data<common_model::GraphCollectionFacade>, req_body: String) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/")]
async fn hi() -> impl Responder {
    HttpResponse::Ok().body("Db Server works! :)")
}

/// initialize common graph collection for all programm lifetime
fn initialize_graph_collection() -> Arc<Mutex<common_model::GraphCollectionFacade>> {
    Arc::new(Mutex::new(common_model::GraphCollectionFacade {
        in_memory_graph_collection: Arc::new(Mutex::new(Vec::new()))
    }))
}