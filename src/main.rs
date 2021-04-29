use std::sync::Arc;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

#[path = "./datamodel/core_model.rs"]
mod core_model;

#[path = "./datamodel/common_model.rs"]
mod common_model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "0.0.0.0:18085";
    print_console_avtan(&url);

    let data = web::Data::new(initialize_graph_collection());

    HttpServer::new( move || {
        App::new()
            .app_data(data.clone())
            .route("/create_graph", web::get().to(create_graph))
            .service(hi)
    })
    .bind(url)?
    .run()
    .await
}

async fn create_graph(data: web::Data<common_model::GraphCollectionFacade>, body: String) -> impl Responder {
    let mut graph_collection = data.in_memory_graph_collection.lock().unwrap();
    let graph = common_model::core_model::InMemoryGraph::new_graph("aaa");
    graph_collection.push(graph);
    let answer  = format!("number is: {} body is \"{}\"", graph_collection.len(), body);
    HttpResponse::Ok().body(answer)
}

/// Healthcheck endpoint
#[get("/avtan")]
async fn hi() -> impl Responder {
    HttpResponse::Ok().body("
                        ~-.
                        ,,,;            ~-.~-.~-
                    (.../           ~-.~-.~-.~-.~-.
                < } O~`, ,        ~-.~-.~-.~-.~-.~-.
                    (/    T ,     ~-.~-.~-.~-.~-.~-.~-.
                        ;    T     ~-.~-.~-.~-.~-.~-.~-.
                      ;   {_.~-.~-.~-.~-.~-.~-.~
                    ;:  .-~`    ~-.~-.~-.~-.~-.
                    ;.: :'    ._   ~-.~-.~-.~-.~-
                    ;::`-.    '-._  ~-.~-.~-.~-
                    ;::. `-.    '-,~-.~-.~-.
                        ';::::.`''-.-'
                        ';::;;:,:'
                            '||T
                            / |
                          __   _
AVTAN DB IS RUNNING!!! KOKOKOKOKOKOKOKOKOKO!!!!! KOKOK!!! POKPOKPOK!!!!!
    ")
}

/// initialize common graph collection for all programm lifetime
fn initialize_graph_collection() -> common_model::GraphCollectionFacade {
    common_model::GraphCollectionFacade {
        in_memory_graph_collection: Arc::new(Mutex::new(Vec::new()))
    }
}

/// Print avtan greeting
fn print_console_avtan(url: &&str)  {
    println!("
                        ░░░░░░░░▄▀▀▄
                        ░░░░░▄▀▒▒▒▒▀▄
                        ░░░░░░▀▌▒▒▐▀ 
                        ▄███▀░◐░░░▌   
                        ░░▐▀▌░░░░░▐░░░░░░░░░▄▀▀▀▄▄
                        ░▐░░▐░░░░░▐░░░░░░░░░█░▄█▀
                        ░▐▄▄▌░░░░░▐▄▄░░░░░░█░░▄▄▀▀▀▀▄
                        ░░░░▌░░░░▄▀▒▒▀▀▀▀▄▀░▄▀░▄▄▄▀▀
                        ░░░▐░░░░▐▒▒▒▒▒▒▒▒▀▀▄░░▀▄▄▄░▄
                        ░░░▐░░░░▐▄▒▒▒▒▒▒▒▒▒▒▀▄░▄▄▀▀
                        ░░░░▀▄░░░░▀▄▒▒▒▒▒▒▒▒▒▒▀▄░
                        ░░░░░▀▄▄░░░█▄▄▄▄▄▄▄▄▄▄▄▀▄
                        ░░░░░░░░▀▀▀▄▄▄▄▄▄▄▄▀▀░
                        ░░░░░░░░░░░▌▌░▌▌
                        ░░░░░░░░░▄▄▌▌▄▌▌
    ");
    println!("                     Avtan server starting on {}", url);
}