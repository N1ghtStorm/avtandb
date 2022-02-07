mod core_model;
mod kv_model;
mod api;
mod kv_api;
mod core_model_tests;
mod sharded_kv_graph;
mod ws;
mod kv_ws;

use std::sync::Arc;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::RwLock;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "0.0.0.0:18085";
    print_console_avtan(&url);

    // CREATE GLOBAL STATE INITIALIZING GRAPH COLLECTION AND KV COLLECTION
    let app_state = web::Data::new( AppState::new());

    // START HTTP SERVER WITH GLOBAL STATE
    HttpServer::new( move || {  
        App::new()
            .app_data(app_state.clone())
            .app_data(actix_send_websocket::WsConfig::new().disable_heartbeat())
            // TEST ENDPOINTS
            .route("/get_test_val", web::get().to(api::get_test_val_by_key))
            .route("/get_graph", web::post().to(api::create_graph))
            .route("/ws/", web::get().to(ws::index))
            // KV - STORE:
            .route("/kv/value/{key}", web::post().to(kv_api::create_value))
            .route("/kv/value/{key}", web::get().to(kv_api::get_value))
            .route("/kv/value/{key}", web::put().to(kv_api::update_value))
            .route("/kv/value/{key}", web::delete().to(kv_api::delete_value))
            .route("/kv/get_all_keys", web::get().to(kv_api::get_all_keys))
            .route("/ws/add_kv/", web::get().to(kv_ws::add_kv_ws))
            .route("/ws/get_kv/", web::get().to(kv_ws::get_kv_ws))
            .service(hi)
    })
    .bind(url)?
    .run()
    .await
}

// WRAPPER STRUCT TO PROVIDE GLOBAL STATE
pub struct AppState {
    graph_collection: core_model::GraphCollectionFacade,
    kv_collection: kv_model::InMemoryKVStore
}

impl AppState {
    fn new() -> AppState {
        AppState {graph_collection: AppState::initialize_graph_collection(), kv_collection: AppState::initialize_kv_store()}
     }
    
    /// initialize common graph collection for all programm lifetime
    fn initialize_graph_collection() -> core_model::GraphCollectionFacade {
        core_model::GraphCollectionFacade {
            in_memory_graph_collection: Arc::new(RwLock::new(Vec::new()))
        }
    }
    
    // initialize kv store for all programm lifetime
    fn initialize_kv_store() -> kv_model::InMemoryKVStore {
        kv_model::InMemoryKVStore::new()
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
    println!("                           🦀🐔🐔🐔🐔🐔🐔🐔🐔🦀
                    Avtan server starting on {}", url);
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
           AVTAN DB IS RUNNING!!! KOKOKOKO!!!!! ;)
    ")
}