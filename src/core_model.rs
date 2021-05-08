use std::sync::Arc;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use actix_web::web;

#[derive(Serialize, Deserialize)]
pub struct CreateGraphDTO {
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct ReturnNodeDTO {
    pub id: u32,
    pub label: String,
    pub bonds: Option<Vec<ReturnBondDTO>>
}

#[derive(Serialize, Deserialize)]
pub struct ReturnBondDTO {
    pub id: u32,
    pub label: String,
    pub src: u32,
    pub dst: u32
}

pub trait Graph {
    fn create_node(&mut self, node: Node) -> Result<(), ()>;
    fn create_bond(&mut self, bond: Bond) -> Result<(), ()>;
} 
/// Main Graph Model
#[derive(Debug)]
pub struct InMemoryGraph {
    pub name: String,
    pub nodes_collection: Vec<Node>,
    pub bonds_collection: Vec<Bond>
}    
    
/// Main Node(Vertex) document collection element 
#[derive(Debug)]
pub struct Node {
    pub id: u32,
    pub label: String
    // TODO Create properties as JSON document
}

/// Main Bond(Relation) document collection element
#[derive(Debug)]
pub struct Bond {
    pub id: u32,
    pub label: String,
    pub src: u32,
    pub dst: u32
}


impl InMemoryGraph {
    /// Creates new empty Graph
    pub fn new_graph(name: String) -> Self {
        InMemoryGraph {name, 
                    nodes_collection: Vec::new(), 
                    bonds_collection: Vec::new()}
    }

    // Maps new empty Graph from DTO
    pub fn new_graph_from_dto(dto: CreateGraphDTO) -> Self {
        todo!()
    }


    /// Add Node to Graph
    pub fn add_node(&mut self, mut node: Node) -> Result<(), ()> {
        if node.label.trim().is_empty() {
            return Err(());
        }

        // Create array of existing indexes
        let mut id_vec: Vec<u32> = self.nodes_collection.iter()
                                                                .map(|x| x.id)
                                                                .collect();

        if node.id == 0 {
            node.id = helpers::get_lowest_unexisting_number(&mut id_vec);
        }
        
        // TODO - CHANGE TO SEARCH TREE VALIDATION TO IMPROVE PERFOMANCE
        if id_vec.iter().any(|&x | x == node.id){
            return Err(());
        }

        self.nodes_collection.push(node);
        Ok(())
    }

    /// Add Bond to Graph
    fn add_bond(&mut self, mut bond: Bond) -> Result<(), ()> {
        if bond.src == 0 || bond.dst == 0 {
            return Err(());
        }

        // Check if bond label not empty
        if bond.label.trim().is_empty() {
            return Err(());
        }

        // Check if src and dst exist in nodes:
        let is_src_exists = self.nodes_collection.iter().any(|x| x.id == bond.src);
        let is_dst_exists = self.nodes_collection.iter().any(|x| x.id == bond.dst);

        if !is_src_exists || !is_dst_exists {
            return Err(());
        }

        // Generate bond id
        let mut id_vec: Vec<u32> = self.bonds_collection.iter()
                                            .map(|x| x.id)
                                            .collect();

        bond.id = helpers::get_lowest_unexisting_number(&mut id_vec);

        self.bonds_collection.push(bond);
        Ok(())
    }

    fn get_connected_nodes_by_depth(&self, node_id: u32, depth: u32){
        todo!();
    }

    fn get_paths_between_ids(&self, start_id: u32, finish_id: u32) -> Result<Vec<Vec<u32>>, ()>{
        let paths = Vec::new();


        Ok(paths)
    }

    /// Drops Whole Graph
    pub fn delete_graph(self){
        todo!();
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

impl Node {
    fn new(id: u32, label: String) -> Self {
        Node {id, label}
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

mod helpers {
    pub fn get_lowest_unexisting_number(vector: &mut Vec<u32>) -> u32 {
        vector.sort();
        let len = vector.len();

        if len < 1 || vector[0] != 1 {
            return 1;    
        } 

        let mut prev = 1;
        for i in  1..len {
            if (vector[i] - prev) > 1 {
                return prev + 1;
            }
            prev = vector[i];
        }

        return vector[len - 1] + 1;
    }
}


//======================================================================================================================
//======================================================================================================================
//======================================================================================================================
// TESTS:
#[cfg(test)]
mod in_memory_graph_tests {
    use std::sync::Arc;
    use std::sync::Mutex;
    use actix_web::web;

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

        {
            let graph_collection_lock = data.in_memory_graph_collection.lock();
            let mut graph_collection = graph_collection_lock.unwrap();
            graph_collection.push(super::InMemoryGraph::new_graph(String::from("some")));
            graph_collection.push(super::InMemoryGraph::new_graph(String::from("some2")));
        }

        let result = super::validate_and_map_graph(dto, data.clone());
        assert_eq!(true, result.is_ok());
        assert_eq!("my_new_graph_name", result.unwrap().name);
    }

    #[test]
    fn validate_and_map_graph_success_failed() {
        let data = web::Data::new(initialize_graph_collection());
        let dto = super::CreateGraphDTO {name: String::from("my_new_graph_name")};

        {
            let graph_collection_lock = data.in_memory_graph_collection.lock();
            let mut graph_collection = graph_collection_lock.unwrap();
            graph_collection.push(super::InMemoryGraph::new_graph(String::from("some")));
            graph_collection.push(super::InMemoryGraph::new_graph(String::from("my_new_graph_name")));
        }

        let result = super::validate_and_map_graph(dto, data.clone());
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn add_node_to_empty_graph_passed() {
        let mut in_mem_graph = super::InMemoryGraph{name: String::from("MyGraph"), nodes_collection: Vec::new(), bonds_collection: Vec::new()};
        let node = super::Node {id: 0, label: String::from("red")};
        let adding_result = in_mem_graph.add_node(node);

        assert_eq!(true, adding_result.is_ok());
        assert_eq!(1, in_mem_graph.nodes_collection.len());
    }

    #[test]
    fn add_node_to_non_empty_graph_passed() {
        let mut in_mem_graph = super::InMemoryGraph{name: String::from("MyGraph"), nodes_collection: Vec::new(), bonds_collection: Vec::new()};

        in_mem_graph.nodes_collection.push(super::Node {id: 1, label: String::from("blue")});
        in_mem_graph.nodes_collection.push(super::Node {id: 3, label: String::from("green")});

        let addong_node = super::Node {id: 0, label: String::from("red")};
        let adding_result = in_mem_graph.add_node(addong_node);

        let added_nodes:Vec<u32> = in_mem_graph.nodes_collection.iter()
                                                               .filter(|x| x.label == String::from("red"))
                                                               .map(|x| x.id)
                                                               .collect();

        let index = added_nodes[0];

        assert_eq!(true, adding_result.is_ok());
        assert_eq!(3, in_mem_graph.nodes_collection.len());
        assert_eq!(2, index);
        assert_eq!(1, added_nodes.len());
    }

    #[test]
    fn add_node_to_non_empty_graph_not_zero_id_passed() {
        let mut in_mem_graph = super::InMemoryGraph{name: String::from("MyGraph"), nodes_collection: Vec::new(), bonds_collection: Vec::new()};

        in_mem_graph.nodes_collection.push(super::Node {id: 1, label: String::from("blue")});
        in_mem_graph.nodes_collection.push(super::Node {id: 3, label: String::from("green")});

        let addong_node = super::Node {id: 4, label: String::from("red")};
        let adding_result = in_mem_graph.add_node(addong_node);

        let added_nodes:Vec<u32> = in_mem_graph.nodes_collection.iter()
                                                               .filter(|x| x.label == String::from("red"))
                                                               .map(|x| x.id)
                                                               .collect();

        let index = added_nodes[0];

        assert_eq!(true, adding_result.is_ok());
        assert_eq!(3, in_mem_graph.nodes_collection.len());
        assert_eq!(4, index);
        assert_eq!(1, added_nodes.len());
    }

    #[test]
    fn add_node_to_non_empty_graph_id_exists_failed() {
        let mut in_mem_graph = super::InMemoryGraph{name: String::from("MyGraph"), nodes_collection: Vec::new(), bonds_collection: Vec::new()};

        in_mem_graph.nodes_collection.push(super::Node {id: 1, label: String::from("blue")});
        in_mem_graph.nodes_collection.push(super::Node {id: 3, label: String::from("green")});

        let adding_node = super::Node {id: 1, label: String::from("red")};
        let adding_result = in_mem_graph.add_node(adding_node);

        let is_node_added = in_mem_graph.nodes_collection.iter()
                                                            .any(|x| x.label == String::from("red"));

        assert_eq!(true, adding_result.is_err());
        assert_eq!(false, is_node_added);
        assert_eq!(2, in_mem_graph.nodes_collection.len());
    }

    #[test]
    fn add_node_blank_label_failed() {
        let mut in_mem_graph = super::InMemoryGraph{name: String::from("MyGraph"), nodes_collection: Vec::new(), bonds_collection: Vec::new()};

        in_mem_graph.nodes_collection.push(super::Node {id: 1, label: String::from("blue")});
        in_mem_graph.nodes_collection.push(super::Node {id: 3, label: String::from("green")});

        let adding_node = super::Node {id: 1, label: String::from("")};
        let adding_result = in_mem_graph.add_node(adding_node);

        let is_node_added = in_mem_graph.nodes_collection.iter()
                                                            .any(|x| x.label == String::from(""));

        assert_eq!(true, adding_result.is_err());
        assert_eq!(false, is_node_added);
        assert_eq!(2, in_mem_graph.nodes_collection.len());
    }

    #[test]
    fn add_node_space_label_failed() {
        let mut in_mem_graph = super::InMemoryGraph{name: String::from("MyGraph"), nodes_collection: Vec::new(), bonds_collection: Vec::new()};

        in_mem_graph.nodes_collection.push(super::Node {id: 1, label: String::from("blue")});
        in_mem_graph.nodes_collection.push(super::Node {id: 3, label: String::from("green")});

        let adding_node = super::Node {id: 2, label: String::from(" ")};
        let adding_result = in_mem_graph.add_node(adding_node);

        let is_node_added = in_mem_graph.nodes_collection.iter()
                                                            .any(|x| x.label == String::from(" "));

        assert_eq!(true, adding_result.is_err());
        assert_eq!(false, is_node_added);
        assert_eq!(2, in_mem_graph.nodes_collection.len());
    }

    #[test]
    fn add_bonds_to_graph_passed() {
        let mut in_mem_graph = super::InMemoryGraph{name: String::from("MyGraph"), nodes_collection: Vec::new(), bonds_collection: Vec::new()};
        in_mem_graph.nodes_collection.push(super::Node {id: 1, label: String::from("blue")});
        in_mem_graph.nodes_collection.push(super::Node {id: 2, label: String::from("green")});
        in_mem_graph.nodes_collection.push(super::Node {id: 3, label: String::from("green")});
        in_mem_graph.nodes_collection.push(super::Node {id: 4, label: String::from("green")});
        in_mem_graph.nodes_collection.push(super::Node {id: 5, label: String::from("blue")});

        in_mem_graph.bonds_collection.push(super::Bond {label: String::from("green-green"), src: 2, dst: 4, id: 0});
        in_mem_graph.bonds_collection.push(super::Bond {label: String::from("green-green"), src: 3, dst: 2, id: 0});
        in_mem_graph.bonds_collection.push(super::Bond {label: String::from("green-green"), src: 1, dst: 5, id: 0});

        let adding_bond = super::Bond {label: String::from("green-green"), src: 1, dst: 2, id: 0};
        let adding_result = in_mem_graph.add_bond(adding_bond);

        assert_eq!(true, adding_result.is_ok());
        assert_eq!(4, in_mem_graph.bonds_collection.len());
    }


    #[test]
    fn add_bonds_to_graph_non_existing_node_failed() {
        let mut in_mem_graph = super::InMemoryGraph{name: String::from("MyGraph"), nodes_collection: Vec::new(), bonds_collection: Vec::new()};
        in_mem_graph.nodes_collection.push(super::Node {id: 1, label: String::from("blue")});
        in_mem_graph.nodes_collection.push(super::Node {id: 2, label: String::from("green")});
        in_mem_graph.nodes_collection.push(super::Node {id: 3, label: String::from("green")});
        in_mem_graph.nodes_collection.push(super::Node {id: 4, label: String::from("green")});
        in_mem_graph.nodes_collection.push(super::Node {id: 5, label: String::from("blue")});

        let adding_bond = super::Bond {label: String::from("green-green"), src: 10, dst: 2, id: 0};
        let adding_result = in_mem_graph.add_bond(adding_bond);

        assert_eq!(true, adding_result.is_err());
        assert_eq!(0, in_mem_graph.bonds_collection.len());
    }

    #[test]
    fn add_bonds_to_graph_empty_label_failed() {
        let mut in_mem_graph = super::InMemoryGraph{name: String::from("MyGraph"), nodes_collection: Vec::new(), bonds_collection: Vec::new()};
        in_mem_graph.nodes_collection.push(super::Node {id: 1, label: String::from("blue")});
        in_mem_graph.nodes_collection.push(super::Node {id: 2, label: String::from("green")});
        in_mem_graph.nodes_collection.push(super::Node {id: 3, label: String::from("green")});
        in_mem_graph.nodes_collection.push(super::Node {id: 4, label: String::from("green")});
        in_mem_graph.nodes_collection.push(super::Node {id: 5, label: String::from("blue")});


        let adding_bond = super::Bond {label: String::from(" "), src: 1, dst: 2, id: 0};
        let adding_result = in_mem_graph.add_bond(adding_bond);

        assert_eq!(true, adding_result.is_err());
        assert_eq!(0, in_mem_graph.bonds_collection.len());
    }
}

#[cfg(test)]
mod helper_tests {
    use super::helpers;

    #[test]
    fn get_allready_sorted_vec_passed() {
        let mut vector = vec![1, 2, 3, 4, 5, 6];
        let last = helpers::get_lowest_unexisting_number(&mut vector);
        assert_eq!(7, last);
    }

    #[test]
    fn get_unsorted_vec_passed() {
        let mut vector = vec![4, 1, 55, 2, 5, 3, 6, 8, 9];
        let last = helpers::get_lowest_unexisting_number(&mut vector);
        assert_eq!(7, last);
    }

    #[test]
    fn get_without_1_passed() {
        let mut vector = vec![4, 55, 2, 5, 3, 6, 8, 9];
        let last = helpers::get_lowest_unexisting_number(&mut vector);
        assert_eq!(1, last);
    }

    #[test]
    fn get_from_empty_vec() {
        let mut vector = Vec::<u32>::new();
        let last = helpers::get_lowest_unexisting_number(&mut vector);
        assert_eq!(1, last);
    }
}
