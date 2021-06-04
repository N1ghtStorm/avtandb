use std::sync::Arc;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use serde_json::Result;
use std::collections::HashMap;

mod core_model;
mod kv_model;
mod ruster_cypher;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "0.0.0.0:18085";
    print_console_avtan(&url);

    // CREATE GLOBAL STATE INITIALIZING GRAPH COLLECTION AND KV COLLECTION
    let app_state = web::Data::new( AppState::new());

    // START HHT SERVER WITH GLOBAL STATE
    HttpServer::new( move || {
        App::new()
            .app_data(app_state.clone())
            // TEST ENDPOINTS
            .route("/get_test_val", web::get().to(get_test_val_by_key))
            .route("/get_graph", web::post().to(create_graph))
            .service(hi)
    })
    .bind(url)?
    .run()
    .await
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
            in_memory_graph_collection: Arc::new(Mutex::new(Vec::new()))
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
        kv_model::InMemoryKVStore{kv_hash_map: Arc::new(Mutex::new(hm))}
    }
}

async fn get_test_val_by_key(data: web::Data<AppState>) -> impl Responder {
    let arc_string_value = data.kv_collection.get_value("foo".to_string()).unwrap();
    HttpResponse::Ok().body(format!("{}",arc_string_value))
}

async fn create_graph(data: web::Data<AppState>, body: String) -> impl Responder {
    let deser_result: Result<core_model::CreateGraphDTO> = serde_json::from_str(&body.to_string());
    let dto = match deser_result{
        Ok(s) => s,
        Err(_) => panic!()
    };
          
    return match core_model::validate_and_map_graph(dto, &data.graph_collection) {
        Err(_) => {
            let graph_collection = data.graph_collection.in_memory_graph_collection.lock().unwrap();
            let len = graph_collection.len();
            let answer  = format!("failed creating graph number is: {} body is \"{}\"", len, body);
            HttpResponse::Conflict().body(answer)
        },
        Ok(img) => {
            let mut graph_collection = data.graph_collection.in_memory_graph_collection.lock().unwrap();
            graph_collection.push(img);
            let len = graph_collection.len();
            let answer  = format!("number is: {} body is \"{:?}\"", len, graph_collection);
            HttpResponse::Ok().body(answer)
        }
    };
}

async fn get_whole_graph(data: web::Data<core_model::GraphCollectionFacade>) -> impl Responder {
    let graph_collection = data.in_memory_graph_collection.lock().unwrap();
    let first_graph = &graph_collection[0];
    let len = first_graph.get_graph_nodes_number();
    // let mut node_dto_vec = Vec::with_capacity(len);

    // for i in 0..len {
    //     node_dto_vec.push(core_model::ReturnNodeDTO {id: first_graph.nodes_collection[i].id, 
    //                                 label: String::from("aaa"), 
    //                                     bonds: None });
    // }

    HttpResponse::Conflict().body("")
}

// async fn get_kv_data {

// }

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