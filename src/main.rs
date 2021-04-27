use std::sync::Arc;

#[path = "./datamodel/core_model.rs"]
mod core_model;

#[path = "./datamodel/common_model.rs"]
mod common_model;

fn main() {
    let graph_collection = initialize_graph_collection();
}


fn initialize_graph_collection() -> common_model::GraphCollectionFacade {
    common_model::GraphCollectionFacade {graph_collection: Arc::new(Vec::new())}
}