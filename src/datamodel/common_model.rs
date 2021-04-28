use std::sync::Arc;
use std::sync::Mutex;

pub mod core_model;

pub struct GraphCollectionFacade {
    //pub graph_collection: Arc<Mutex<Vec<Box<dyn core_model::Graph>>>>
    pub in_memory_graph_collection: Arc<Mutex<Vec<core_model::InMemoryGraph>>>
}