use std::sync::Arc;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use actix_web::{web};

#[derive(Serialize, Deserialize)]
pub struct CreateGraphDTO {
    pub name: String
}

pub trait Graph {
    fn create_node(&mut self, node: Node) -> Result<(), ()>;
    fn create_bond(&mut self, bond: Bond) -> Result<(), ()>;
} 
/// Main Graph Model
pub struct InMemoryGraph {
    pub name: String,
    pub nodes_collection: Vec<Node>,
    pub bonds_collection: Vec<Bond>
}    
    
/// Main Node(Vertex) document collection element 
pub struct Node {
    id: u32,
    label: String
    // TODO Create properties as JSON document
}

/// Main Bond(Relation) document collection element
pub struct Bond {
    id: u32,
    label: String,
    src: i32,
    dst: i32
}

impl InMemoryGraph {
    /// Creates new empty Graph
    pub fn new_graph(name: String) -> Self {
        InMemoryGraph {name, 
                    nodes_collection: Vec::new(), 
                    bonds_collection: Vec::new()}
    }

    /// Drops Whole Graph
    pub fn delete_graph(self){
        drop(self);
    }
}
//  Main Graph action Methods impl
impl Graph for InMemoryGraph {
    /// Creates Node, adding to nodes collection
    fn create_node(&mut self, node: Node) -> Result<(), ()> {
        self.nodes_collection.push(node);
        Ok(())
    }

    /// Creates Bond, adding to bonds collection
    fn create_bond(&mut self, bond: Bond) -> Result<(), ()> {
        self.bonds_collection.push(bond);
        Ok(())
    }
}

pub struct GraphCollectionFacade {
    pub in_memory_graph_collection: Arc<Mutex<Vec<InMemoryGraph>>>
}


pub fn validate_and_map_graph(dto: CreateGraphDTO, 
    graph_data: web::Data<GraphCollectionFacade>) -> Result<InMemoryGraph, ()> {
    let graphs = graph_data.in_memory_graph_collection.lock().unwrap();

    // check if exactly name existst
    for i in 0..graphs.len() {
        if dto.name == graphs[i].name {
            return Err(());
        }
    }

    let graph = InMemoryGraph::new_graph(dto.name);
    Ok(graph)
}   







// TESTS:


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::Mutex;
    use actix_web::{web};

    fn initialize_graph_collection() -> super::GraphCollectionFacade {
        super::GraphCollectionFacade {
            in_memory_graph_collection: Arc::new(Mutex::new(Vec::new()))
        }
    }

    #[test]
    fn validate_and_map_graph_passed() {
        
        let data = web::Data::new(initialize_graph_collection());
        let dto = super::CreateGraphDTO {name: String::from("my_new_graph_name")};
        let result = super::validate_and_map_graph(dto, data.clone());

        assert_eq!(true, result.is_ok());
        assert_eq!("my_new_graph_name", result.unwrap().name);
    }

    #[test]
    fn validate_and_map_graph_with_filled_passed() {
        
        let data = web::Data::new(initialize_graph_collection());
        let dto = super::CreateGraphDTO {name: String::from("my_new_graph_name")};
        let mut graph_collection = data.in_memory_graph_collection.lock().unwrap();

        graph_collection.push(super::InMemoryGraph::new_graph(String::from("some")));
        graph_collection.push(super::InMemoryGraph::new_graph(String::from("some2")));

        //data.in_memory_graph_collection.

        let result = super::validate_and_map_graph(dto, data.clone());

        assert_eq!(true, result.is_ok());
        assert_eq!("my_new_graph_name", result.unwrap().name);
    }

    #[test]
    fn validate_and_map_graph_success_failed() {
        
        // let data = web::Data::new(initialize_graph_collection());
        // let dto = super::CreateGraphDTO {name: String::from("my_new_graph_name")};
        // let mut graph_collection = data.in_memory_graph_collection.lock().unwrap();

        // graph_collection.push(super::InMemoryGraph::new_graph(String::from("some")));
        // graph_collection.push(super::InMemoryGraph::new_graph(String::from("my_new_graph_name")));

        // let result = super::validate_and_map_graph(dto, data.clone());

        // assert_eq!(false, result.is_ok());
    }
}
