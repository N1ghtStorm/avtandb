use std::sync::Arc;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use actix_web::web;
use std::collections::BTreeMap;
use uuid::Uuid;

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
    pub id: Uuid,
    pub label: String,
    pub src: Uuid,
    pub dst: Uuid
}

pub trait Graph {
    fn create_node(&mut self, node: Node) -> Result<(), ()>;
    fn create_bond(&mut self, bond: Bond) -> Result<(), ()>;
} 
/// Main Graph Model
#[derive(Debug)]
pub struct InMemoryGraph {
    pub name: String,
    nodes_collection: Vec<Node>,
    bonds_collection: Vec<Bond>,
    nodes_id_index: BTreeMap<Uuid, usize>
}
pub struct GraphCollectionFacade {
    pub in_memory_graph_collection: Arc<Mutex<Vec<InMemoryGraph>>>
}
    
/// Main Node(Vertex) document collection element 
#[derive(Debug)]
pub struct Node {
    pub id: Uuid,
    pub labels: Vec<String>
    // TODO Create properties as JSON document
}

/// Main Bond(Relation) document collection element
#[derive(Debug)]
pub struct Bond {
    pub id: Uuid,
    pub label: String,
    pub src: Uuid,
    pub dst: Uuid
}

#[derive(PartialEq)]
pub enum BondDirection {
    Outgoing,
    Ingoing,
    Both
}


impl InMemoryGraph {
    /// Creates new empty Graph
    pub fn new_graph(name: String) -> Self {
        InMemoryGraph {name, 
                    nodes_collection: Vec::new(), 
                    bonds_collection: Vec::new(),
                    nodes_id_index: BTreeMap::new()}
    }

    // Maps new empty Graph from DTO
    pub fn new_graph_from_dto(dto: CreateGraphDTO) -> Self {
        let a = Uuid::new_v4();
        todo!()
    }


    /// Add Node to Graph
    pub fn add_node(&mut self, mut node: Node) -> Result<(), ()> {
        if node.labels.len() == 0 || node.labels[0].trim().is_empty() {
            return Err(());
        }

        if node.id == Uuid::default() {
            node.id = Uuid::new_v4();
        }

        if self.nodes_id_index.contains_key(&node.id) {
            return Err(());
        }

        let len = self.nodes_collection.len();
        self.nodes_id_index.insert(node.id, len);
        self.nodes_collection.push(node);

        Ok(())
    }

    /// Add Bond to Graph
    fn add_bond(&mut self, mut bond: Bond) -> Result<(), ()> {
        if bond.src == Uuid::default() || bond.dst == Uuid::default() {
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
        let mut id_vec: Vec<Uuid> = self.bonds_collection.iter()
                                            .map(|x| x.id)
                                            .collect();

        bond.id = Uuid::new_v4();

        self.bonds_collection.push(bond);
        Ok(())
    }

    fn get_connected_nodes_by_depth(&self, node_id: Uuid, depth: u32){
        todo!();
    }

    // TODO THINK ABOUT REFACTORING
    pub fn get_connected_nodes(&self, node_id: Uuid, bond_types: Vec<String>, node_labels: Vec<String>, direction: BondDirection) -> Result<Vec<&Node>, ()>{
        let mut nodes_refs = Vec::<&Node>::new();
        let node_index_opt = self.nodes_id_index.get(&node_id);

        if node_index_opt.is_none() {
            return Err(());
        }

        let node_labels_len = node_labels.len();
        let bond_types_len = bond_types.len();

        let curr_node = &self.nodes_collection[*node_index_opt.unwrap()];
        nodes_refs.push(curr_node);

        // If ingoing - skip
        if direction != BondDirection::Ingoing {
            let nodes_by_outgoing_ids: Vec<Uuid> = self.bonds_collection.iter().filter(|x| x.src == node_id && {
                                                                                    if bond_types_len == 0 { true } else {             
                                                                                        bond_types.contains(&x.label)
                                                                                    }
                                                                                })
                                                                                .map(|x| x.dst)
                                                                                .collect();
            for i in 0..nodes_by_outgoing_ids.len() {
                let curr_node_index = self.nodes_id_index.get(&nodes_by_outgoing_ids[i]).unwrap();
                let dst_node = &self.nodes_collection[*curr_node_index];

                // if len is 0 - we include all labels
                if node_labels_len == 0 { 
                    nodes_refs.push(dst_node);
                    break;
                }

                // Add only if labels intersect
                for label in &dst_node.labels {
                    if node_labels.contains(label) {
                        nodes_refs.push(dst_node);
                        break;
                    }
                }
                
            }
        }

        // If outgoing - skip
        if direction != BondDirection::Outgoing {
            let nodes_by_ingoing_ids: Vec<Uuid> = self.bonds_collection.iter().filter(|x| x.dst == node_id && {
                                                                                    if bond_types_len == 0 { true } else {             
                                                                                        bond_types.contains(&x.label)
                                                                                    }
                                                                                })
                                                                              .map(|x| x.src)
                                                                              .collect();
            for i in 0..nodes_by_ingoing_ids.len() {
                let curr_node_index = self.nodes_id_index.get(&nodes_by_ingoing_ids[i]).unwrap();
                let src_node = &self.nodes_collection[*curr_node_index];

                // if len is 0 - we include all labels
                if node_labels_len == 0 { 
                    nodes_refs.push(src_node);
                    break;
                }
                
                // Add only if labels intersect
                for label in &src_node.labels {
                    if node_labels.contains(label) {
                        nodes_refs.push(src_node);
                        break;
                    }
                }
            }
        }

        Ok(nodes_refs)
    }

    fn get_paths_between_ids(&self, start_id: u32, finish_id: u32) -> Result<Vec<Vec<u32>>, ()>{
        let paths = Vec::new();


        Ok(paths)
    }

    /// Drops Whole Graph
    pub fn delete_graph(self){
        todo!();
    }

    pub fn get_graph_nodes_number(&self) -> usize{
        self.nodes_collection.len()
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
    fn new(id: Uuid, labels: Vec<String>) -> Self {
        Node {id, labels}
    }
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

//======================================================================================================================
//======================================================================================================================
//======================================================================================================================
// TESTS:
#[cfg(test)]
mod in_memory_graph_tests {
    use std::sync::Arc;
    use std::sync::Mutex;
    use actix_web::web;
    use uuid::Uuid;

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
        let mut in_mem_graph = super::InMemoryGraph::new_graph("MyGraph".to_string());
        
        let node = super::Node {id: Uuid::default(), labels: vec![String::from("red")]};
        let adding_result = in_mem_graph.add_node(node);

        let node_uuid = in_mem_graph.nodes_collection[0].id;
        let btree_node_id = in_mem_graph.nodes_id_index.get(&node_uuid);

        assert_eq!(0, *btree_node_id.unwrap());
        assert_eq!(true, adding_result.is_ok());
        assert_eq!(1, in_mem_graph.nodes_collection.len());
    }

    #[test]
    fn add_node_to_non_empty_graph_passed() {
        let mut in_mem_graph = super::InMemoryGraph::new_graph("MyGraph".to_string());

        in_mem_graph.nodes_collection.push(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), 
                                                        labels: vec![String::from("blue")]});
        in_mem_graph.nodes_collection.push(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap(), 
                                                        labels: vec![String::from("green")]});

        let addong_node = super::Node {id: Uuid::default(), labels: vec![String::from("red")]};
        let adding_result = in_mem_graph.add_node(addong_node);

        let added_nodes:Vec<Uuid> = in_mem_graph.nodes_collection.iter()
                                                               .filter(|x| x.labels.contains(&String::from("red")))
                                                               .map(|x| x.id)
                                                               .collect();

        let index = added_nodes[0];

        assert_eq!(true, adding_result.is_ok());
        assert_eq!(3, in_mem_graph.nodes_collection.len());
        assert_ne!(Uuid::default(), index);
        assert_eq!(1, added_nodes.len());
    }

    #[test]
    fn add_node_to_non_empty_graph_not_zero_id_passed() {
        let mut in_mem_graph = super::InMemoryGraph::new_graph("MyGraph".to_string());

        let r1 = in_mem_graph.add_node(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), 
                                                                labels: vec![String::from("blue")]});
        let r2 = in_mem_graph.add_node(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap(), 
                                                                labels: vec![String::from("green")]});

        let checking_node_uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let adding_node = super::Node {id: checking_node_uuid, labels: vec![String::from("red")]};
        let adding_result = in_mem_graph.add_node(adding_node);

        let added_nodes:Vec<Uuid> = in_mem_graph.nodes_collection.iter()
                                                               .filter(|x| x.labels.contains(&String::from("red")))
                                                               .map(|x| x.id)
                                                               .collect();

        let index = added_nodes[0];
        let node_vector_index = in_mem_graph.nodes_id_index.get(&checking_node_uuid);

        assert_eq!(2, *node_vector_index.unwrap());
        assert!(r1.is_ok());
        assert!(r2.is_ok());
        assert_eq!(true, adding_result.is_ok());
        assert_eq!(3, in_mem_graph.nodes_collection.len());
        assert_ne!(Uuid::default(), index);
        assert_eq!(1, added_nodes.len());
    }

    #[test]
    fn add_nodes_to_graph_get_correct_index_id_passed() {
        let mut in_mem_graph = super::InMemoryGraph::new_graph("MyGraph".to_string());

        let r1 = in_mem_graph.add_node(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), 
                                                                labels: vec![String::from("blue")]});
        let r2 = in_mem_graph.add_node(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap(), 
                                                                labels: vec![String::from("green")]});
        let r3 = in_mem_graph.add_node(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400300").unwrap(), 
                                                                labels: vec![String::from("green")]});
        let r4 = in_mem_graph.add_node(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap(), 
                                                                labels: vec![String::from("green")]});

        let node_vector_index = in_mem_graph.nodes_id_index.get(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap());

        assert_eq!(3, *node_vector_index.unwrap());
        assert!(r1.is_ok());
        assert!(r2.is_ok());
        assert!(r3.is_ok());
        assert!(r4.is_ok());
    }

    #[test]
    fn add_node_to_non_empty_graph_id_exists_failed() {
        let mut in_mem_graph = super::InMemoryGraph::new_graph("MyGraph".to_string());

        let r1 = in_mem_graph.add_node(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), 
                                                                 labels: vec![String::from("blue")]});
        let r2 = in_mem_graph.add_node(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap(), 
                                                                labels: vec![String::from("green")]});


        let adding_node = super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), labels: vec![String::from("red")]};
        let adding_result = in_mem_graph.add_node(adding_node);

        let is_node_added = in_mem_graph.nodes_collection.iter()
                                                            .any(|x| x.labels.contains(&String::from("red")));
        assert!(r1.is_ok());
        assert!(r2.is_ok());
        assert_eq!(true, adding_result.is_err());
        assert_eq!(false, is_node_added);
        assert_eq!(2, in_mem_graph.nodes_collection.len());
    }

    #[test]
    fn add_node_blank_label_failed() {
        let mut in_mem_graph = super::InMemoryGraph::new_graph("MyGraph".to_string());

        in_mem_graph.nodes_collection.push(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), 
                                                        labels: vec![String::from("blue")]});
        in_mem_graph.nodes_collection.push(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap(), 
                                                        labels: vec![String::from("green")]});

        let adding_node = super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap(), labels: vec![String::from("")]};
        let adding_result = in_mem_graph.add_node(adding_node);

        let is_node_added = in_mem_graph.nodes_collection.iter()
                                                            .any(|x| x.labels.contains(&String::from("")));

        assert_eq!(true, adding_result.is_err());
        assert_eq!(false, is_node_added);
        assert_eq!(2, in_mem_graph.nodes_collection.len());
    }

    #[test]
    fn add_node_space_label_failed() {
        let mut in_mem_graph = super::InMemoryGraph::new_graph("MyGraph".to_string());

        in_mem_graph.nodes_collection.push(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), labels: vec![String::from("blue")]});
        in_mem_graph.nodes_collection.push(super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap(), labels: vec![String::from("green")]});

        let adding_node = super::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap(), labels: vec![String::from(" ")]};
        let adding_result = in_mem_graph.add_node(adding_node);

        let is_node_added = in_mem_graph.nodes_collection.iter()
                                                            .any(|x| x.labels.contains(&String::from(" ")));

        assert_eq!(true, adding_result.is_err());
        assert_eq!(false, is_node_added);
        assert_eq!(2, in_mem_graph.nodes_collection.len());
    }

    #[test]
    fn add_bonds_to_graph_passed() {
        let mut in_mem_graph = super::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.nodes_collection.push(super::Node {id: uuid_1, labels: vec![String::from("blue")]});
        in_mem_graph.nodes_collection.push(super::Node {id: uuid_2, labels: vec![String::from("green")]});
        in_mem_graph.nodes_collection.push(super::Node {id: uuid_3, labels: vec![String::from("green")]});
        in_mem_graph.nodes_collection.push(super::Node {id: uuid_4, labels: vec![String::from("green")]});
        in_mem_graph.nodes_collection.push(super::Node {id: uuid_5, labels: vec![String::from("blue")]});

        in_mem_graph.bonds_collection.push(super::Bond {label: String::from("green-green"), src: uuid_2, dst: uuid_4, id: Uuid::new_v4()});
        in_mem_graph.bonds_collection.push(super::Bond {label: String::from("green-green"), src: uuid_3, dst: uuid_2, id: Uuid::new_v4()});
        in_mem_graph.bonds_collection.push(super::Bond {label: String::from("green-green"), src: uuid_1, dst: uuid_5, id: Uuid::new_v4()});

        let adding_bond = super::Bond {label: String::from("green-green"), src: uuid_1, dst: uuid_2, id: Uuid::new_v4()};
        let adding_result = in_mem_graph.add_bond(adding_bond);

        assert_eq!(true, adding_result.is_ok());
        assert_eq!(4, in_mem_graph.bonds_collection.len());
    }


    #[test]
    fn add_bonds_to_graph_non_existing_node_failed() {
        let mut in_mem_graph = super::InMemoryGraph::new_graph("MyGraph".to_string());
        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();
        
        in_mem_graph.nodes_collection.push(super::Node {id: uuid_1, labels: vec![String::from("blue")]});
        in_mem_graph.nodes_collection.push(super::Node {id: uuid_2, labels: vec![String::from("green")]});
        in_mem_graph.nodes_collection.push(super::Node {id: uuid_3, labels: vec![String::from("green")]});
        in_mem_graph.nodes_collection.push(super::Node {id: uuid_4, labels: vec![String::from("green")]});
        in_mem_graph.nodes_collection.push(super::Node {id: uuid_5, labels: vec![String::from("blue")]});

        let adding_bond = super::Bond {label: String::from("green-green"), src: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400010").unwrap(), 
                                        dst: uuid_2, id: Uuid::new_v4()};
        let adding_result = in_mem_graph.add_bond(adding_bond);

        assert_eq!(true, adding_result.is_err());
        assert_eq!(0, in_mem_graph.bonds_collection.len());
    }

    #[test]
    fn add_bonds_to_graph_empty_label_failed() {
        let mut in_mem_graph = super::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.nodes_collection.push(super::Node {id: uuid_1, labels: vec![String::from("blue")]});
        in_mem_graph.nodes_collection.push(super::Node {id: uuid_2, labels: vec![String::from("green")]});
        in_mem_graph.nodes_collection.push(super::Node {id: uuid_3, labels: vec![String::from("green")]});
        in_mem_graph.nodes_collection.push(super::Node {id: uuid_4, labels: vec![String::from("green")]});
        in_mem_graph.nodes_collection.push(super::Node {id: uuid_5, labels: vec![String::from("blue")]});


        let adding_bond = super::Bond {label: String::from(" "), src: uuid_1, dst: uuid_2, id: Uuid::new_v4()};
        let adding_result = in_mem_graph.add_bond(adding_bond);

        assert_eq!(true, adding_result.is_err());
        assert_eq!(0, in_mem_graph.bonds_collection.len());
    }


    #[test]
    fn get_simple_connected_nodes_passed() {
        let mut in_mem_graph = super::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.add_node(super::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(super::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(super::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(super::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(super::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        in_mem_graph.add_bond(super::Bond {label: String::from("green-green"), src: uuid_2, dst: uuid_4, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(super::Bond {label: String::from("green-green"), src: uuid_3, dst: uuid_2, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(super::Bond {label: String::from("green-green"), src: uuid_1, dst: uuid_5, id: Uuid::new_v4()}).unwrap();

        let connected_nodes_with_2 = in_mem_graph.get_connected_nodes(uuid_2, Vec::new(), Vec::new(), super::BondDirection::Both).unwrap();
        let conn_nodes_ids_with_2: Vec<Uuid> = connected_nodes_with_2.iter().map(|x| x.id).collect();

        assert_eq!(3, connected_nodes_with_2.len());
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap()));
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap()));
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap()));
    }

    #[test]
    fn get_connected_nodes_with_labels_passed() {
        let mut in_mem_graph = super::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.add_node(super::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(super::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(super::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(super::Node {id: uuid_4, labels: vec![String::from("grey")]}).unwrap();
        in_mem_graph.add_node(super::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        in_mem_graph.add_bond(super::Bond {label: String::from("green-green"), src: uuid_2, dst: uuid_4, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(super::Bond {label: String::from("green-green"), src: uuid_3, dst: uuid_2, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(super::Bond {label: String::from("green-green"), src: uuid_1, dst: uuid_5, id: Uuid::new_v4()}).unwrap();

        let connected_nodes_with_2 = in_mem_graph.get_connected_nodes(uuid_2, 
                                    Vec::new(), 
                                    vec!["green".to_string()], 
                                    super::BondDirection::Both).unwrap();

        let conn_nodes_ids_with_2: Vec<Uuid> = connected_nodes_with_2.iter().map(|x| x.id).collect();

        assert_eq!(2, connected_nodes_with_2.len());
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap()));
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap()));
        assert!(!conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap()));
    }

    #[test]
    fn get_connected_nodes_with_bond_label_passed() {
        let mut in_mem_graph = super::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.add_node(super::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(super::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(super::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(super::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(super::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        in_mem_graph.add_bond(super::Bond {label: String::from("green-grey"), src: uuid_2, dst: uuid_4, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(super::Bond {label: String::from("green-green"), src: uuid_3, dst: uuid_2, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(super::Bond {label: String::from("green-green"), src: uuid_1, dst: uuid_5, id: Uuid::new_v4()}).unwrap();

        let connected_nodes_with_2 = in_mem_graph.get_connected_nodes(uuid_2, 
                            vec!["green-green".to_string()],
                                    Vec::new(), 
                                    super::BondDirection::Both).unwrap();

        let conn_nodes_ids_with_2: Vec<Uuid> = connected_nodes_with_2.iter().map(|x| x.id).collect();

        assert_eq!(2, connected_nodes_with_2.len());
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap()));
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap()));
        assert!(!conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap()));
    }
}