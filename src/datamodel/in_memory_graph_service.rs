// use actix_web::{web};

// #[path = "../datamodel/core_model.rs"]
// mod core_model;

// #[path = "../apimodel/manage_graph_models.rs"]
// pub mod manage_graph_models;

// #[path = "../datamodel/common_model.rs"]
// pub mod common_model;


// pub fn create_graph(dto: manage_graph_models::CreateGraphDTO, 
//                 graph_data: web::Data<common_model::GraphCollectionFacade>) -> core_model::InMemoryGraph {
//     //let existing_graph_names = graph_data.in_memory_graph_collection.lock().unwrap().iter().map(|x| x.name);
//     let graph = core_model::InMemoryGraph::new_graph("aaa");
//     graph
// }   