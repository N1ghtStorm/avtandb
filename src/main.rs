use std::sync::Arc;

#[path = "./datamodel/core_model.rs"]
mod core_model;

#[path = "./datamodel/common_model.rs"]
mod common_model;

fn main() {
    let mut graph_collection = initialize_graph_collection();
}

fn initialize_graph_collection() -> Arc<common_model::GraphCollectionFacade> {
    Arc::new(common_model::GraphCollectionFacade {graph_collection: Arc::new(Vec::new())})
}