use crate::core_model;
use crate::{AppState};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::Result;

pub async fn get_test_val_by_key(data: web::Data<AppState>) -> impl Responder {
    let arc_string_value = data.kv_collection.get_value("foo".to_string()).unwrap();
    HttpResponse::Ok().body(format!("{}",arc_string_value))
}

pub async fn create_graph(data: web::Data<AppState>, body: String) -> impl Responder {
    let deser_result: Result<core_model::CreateGraphDTO> = serde_json::from_str(&body.to_string());
    let dto = match deser_result{
        Ok(s) => s,
        Err(_) => panic!()
    };
          
    match core_model::validate_and_map_graph(dto, &data.graph_collection) {
        Err(_) => {
            let graph_collection = data.graph_collection.in_memory_graph_collection.read().unwrap();
            let len = graph_collection.len();
            let answer  = format!("failed creating graph number is: {} body is \"{}\"", len, body);
            HttpResponse::Conflict().body(answer)
        },
        Ok(img) => {
            let mut graph_collection = data.graph_collection.in_memory_graph_collection.write().unwrap();
            graph_collection.push(img);
            let len = graph_collection.len();
            let answer  = format!("number is: {} body is \"{:?}\"", len, graph_collection);
            HttpResponse::Ok().body(answer)
        }
    }
}


async fn get_whole_graph(data: web::Data<core_model::GraphCollectionFacade>) -> impl Responder {
    let graph_collection = data.in_memory_graph_collection.read().unwrap();
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