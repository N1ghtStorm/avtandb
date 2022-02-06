use std::sync::Arc;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::RwLock;
use std::collections::HashMap;

mod core_model;
mod kv_model;
mod api;
mod core_model_tests;
mod sharded_kv_graph;

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
            // TEST ENDPOINTS
            .route("/get_test_val", web::get().to(api::get_test_val_by_key))
            .route("/get_graph", web::post().to(api::create_graph))
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
        AppState {graph_collection: AppState::initialize_graph_collection(), kv_collection: AppState::initialize_test_kv_store()}
     }
    
    /// initialize common graph collection for all programm lifetime
    fn initialize_graph_collection() -> core_model::GraphCollectionFacade {
        core_model::GraphCollectionFacade {
            in_memory_graph_collection: Arc::new(RwLock::new(Vec::new()))
        }
    }
    
    // initialize kv store for all programm lifetime
    fn initialize_test_kv_store() -> kv_model::InMemoryKVStore {
        //kv_model::KVStore::new()
        let key_1 = "foo".to_string();
        let val_1 = 
        "{
            \"a\": 1,
            \"b\": \"asd\",
            \"arr\": [{},{},{\"lol\": 20}]
        } ".to_string();
        let mut hm = HashMap::new();
        hm.insert(key_1, Arc::new(val_1));
        kv_model::InMemoryKVStore{kv_hash_map: Arc::new(tokio::sync::RwLock::new(hm))}
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