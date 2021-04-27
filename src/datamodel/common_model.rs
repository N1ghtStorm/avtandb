use std::sync::Arc;

mod core_model;

pub struct GraphCollectionFacade {
    pub graph_collection: Arc<Vec<Box<dyn core_model::Graph>>>
}