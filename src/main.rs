use std::sync::Arc;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use serde_json::Result;
use std::collections::HashMap;

mod core_model;
mod kv_model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "0.0.0.0:18085";
    print_console_avtan(&url);

    let graph_data = web::Data::new(initialize_graph_collection());
    let kv_data = web::Data::new(initialize_kv_store());

    HttpServer::new( move || {
        App::new()
            .app_data(kv_data.clone())
            .route("/get_test_val", web::get().to(get_test_val_by_key))


            // .app_data(graph_data.clone())
            // .route("/create_graph", web::get().to(create_graph))
            // .app_data(data.clone())
            // .route("/get_graph", web::get().to(create_graph))
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
AVTAN DB IS RUNNING!!! KOKOKOKO!!!!! KOKOK!!! 🦀🐔🐔🐔🐔🐔🐔🐔🐔🦀!!!!!
    ")
}

// /// Get
// #[post("/get_whole_graph")]
// async fn get_whole_graph() -> impl Responder {
//     HttpResponse::Ok().body("")
// }

/// Main Endpoint for command
#[post("/command")]
async fn execute_command() -> impl Responder {
    HttpResponse::Ok().body("")
}

/// initialize common graph collection for all programm lifetime
fn initialize_graph_collection() -> core_model::GraphCollectionFacade {
    core_model::GraphCollectionFacade {
        in_memory_graph_collection: Arc::new(Mutex::new(Vec::new()))
    }
}

// initialize kv store for all programm lifetime
fn initialize_kv_store() -> kv_model::KVStore {
    //kv_model::KVStore::new()
    let key_1 = "foo".to_string();
    let val_1 = 
    "
        {
            \"a\": 1,
            \"b\": \"asd\",
            \"arr\": [{},{},{\"lol\": 20}]
        }
    
    ".to_string();
    let mut hm = HashMap::new();
    hm.insert(key_1, val_1);
    kv_model::KVStore{kv_hash_map: Arc::new(Mutex::new(hm))}
}

async fn get_test_val_by_key(data: web::Data<kv_model::KVStore>) -> impl Responder {
    let aaa = data.get_value("foo".to_string()).unwrap();
    HttpResponse::Ok().body(aaa)
}

async fn create_graph(data: web::Data<core_model::GraphCollectionFacade>, body: String) -> impl Responder {
    let deser_result: Result<core_model::CreateGraphDTO> = serde_json::from_str(&body.to_string());
    let dto = deser_result.unwrap();
          
    return match core_model::validate_and_map_graph(dto, data.clone()) {
        Err(_) => {
            let graph_collection = data.in_memory_graph_collection.lock().unwrap();
            let len = graph_collection.len();
            let answer  = format!("failed creating graph number is: {} body is \"{}\"", len, body);
            HttpResponse::Conflict().body(answer)
        },
        Ok(img) => {
            let mut graph_collection = data.in_memory_graph_collection.lock().unwrap();
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
    let len = first_graph.nodes_collection.len();
    let mut node_dto_vec = Vec::with_capacity(len);

    for i in 0..len {
        node_dto_vec.push(core_model::ReturnNodeDTO {id: first_graph.nodes_collection[i].id, 
                                    label: String::from("aaa"), 
                                        bonds: None });
    }

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
    println!("                     Avtan server starting on {}", url);
}