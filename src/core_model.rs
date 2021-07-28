use std::sync::{Arc, Mutex, RwLock};
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use uuid::Uuid;
use std::collections::HashSet;

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
    pub nodes_collection: Vec<Node>,
    pub bonds_collection: Vec<Bond>,
    pub nodes_id_index: BTreeMap<Uuid, usize>,
    pub bonds_id_index: BTreeMap<Uuid, usize>
}
pub struct GraphCollectionFacade {
    pub in_memory_graph_collection: Arc<RwLock<Vec<InMemoryGraph>>>
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
                    nodes_id_index: BTreeMap::new(),
                    bonds_id_index: BTreeMap::new()
                }
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
    pub fn add_bond(&mut self, mut bond: Bond) -> Result<(), ()> {
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

    /// SHITTY CODE - REFACTOR!!!!!!!!!!!!!
    /// GETS CONNECTED NODES WITH CURRENT
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
            add_outgoing_nodes(&self, node_id, &bond_types, &node_labels,  &mut nodes_refs, node_labels_len, bond_types_len);
        }

        // If outgoing - skip
        if direction != BondDirection::Outgoing {
            add_ingoing_nodes(&self, node_id, &bond_types, &node_labels,  &mut nodes_refs, node_labels_len, bond_types_len);
        }

        return Ok(nodes_refs);

        // INNER FUNCTIONS:
        /// Get nodes nodes by outgoing bonds
        fn add_outgoing_nodes<'a>(self_graph: &'a InMemoryGraph, node_id: Uuid, bond_types: &Vec<String>, node_labels: &Vec<String>, 
                                                            nodes_refs: &mut Vec<&'a Node>, node_labels_len: usize, bond_types_len: usize) {

            // get id by outgoing                                              
            let nodes_by_outgoing_ids: Vec<Uuid> = self_graph.bonds_collection.iter()
                                                                                .filter(|x| x.src == node_id && {
                                                                                    if bond_types_len == 0 { true } else {             
                                                                                        bond_types.contains(&x.label)
                                                                                    }
                                                                                })
                                                                                .map(|x| x.dst)
                                                                                .collect();


            for i in 0..nodes_by_outgoing_ids.len() {
                let curr_node_index = self_graph.nodes_id_index.get(&nodes_by_outgoing_ids[i]).unwrap();
                let dst_node = &self_graph.nodes_collection[*curr_node_index];

                // if len is 0 - we include all labels
                if node_labels_len == 0 { 
                    nodes_refs.push(dst_node);
                    continue;
                }

                // Add only if labels intersect
                for label in &dst_node.labels {
                    if node_labels.contains(label) {
                        nodes_refs.push(dst_node);
                        continue;
                    }
                }  
            }
        }
        
        /// Get nodes nodes by ingoing bonds
        fn add_ingoing_nodes<'a>(self_graph: &'a InMemoryGraph, node_id: Uuid, bond_types: &Vec<String>, node_labels: &Vec<String>, 
                                                        nodes_refs: &mut Vec<&'a Node>, node_labels_len: usize, bond_types_len: usize) {
            let nodes_by_ingoing_ids: Vec<Uuid> = self_graph.bonds_collection.iter()
                                                                                .filter(|x| x.dst == node_id && {
                                                                                        if bond_types_len == 0 { true } else {             
                                                                                            bond_types.contains(&x.label)
                                                                                        }
                                                                                    })
                                                                                .map(|x| x.src)
                                                                                .collect();
            for i in 0..nodes_by_ingoing_ids.len() {
                let curr_node_index = self_graph.nodes_id_index.get(&nodes_by_ingoing_ids[i]).unwrap();
                let src_node = &self_graph.nodes_collection[*curr_node_index];

                // if len is 0 - we include all labels
                if node_labels_len == 0 { 
                    nodes_refs.push(src_node);
                    continue;
                }
                
                // Add only if labels intersect
                for label in &src_node.labels {
                    if node_labels.contains(label) {
                        nodes_refs.push(src_node);
                        continue;
                    }
                }
            }
        }
    }

    /// GETS NODES THAT EXIST IN UUID LIST
    pub fn get_nodes_by_id_list(&self, uuid_list: Vec<Uuid>) -> Result<Vec<&Node>, ()>{
        let mut existing_node_refs = Vec::new();
        let mut existing_uuids_set = HashSet::<Uuid>::new();
        
        // CHECK IF EXISTS => THEN ADD TO RETURN VECTOR
        for i in 0..uuid_list.len()  {
            match self.nodes_id_index.get(&uuid_list[i]){
                Some(index_ref) => {
                    if !existing_uuids_set.contains(&self.nodes_collection[*index_ref].id){
                        let node_ref = &self.nodes_collection[*index_ref];
                        existing_node_refs.push(node_ref);
                        existing_uuids_set.insert(self.nodes_collection[*index_ref].id);
                    }
                },
                None => ()
            }
        }

        Ok(existing_node_refs)
    }

    /// GETS NODES THAT EXIST IN LABEL LIST
    pub fn get_nodes_by_label_list(&self, label_list: Vec<String>) -> Result<Vec<&Node>, ()>{
        let mut existing_node_refs = Vec::new();
        let mut existing_uuids_set = HashSet::<Uuid>::new();
        
        // CHECK IF EXISTS => THEN ADD TO RETURN VECTOR
        // CURRENTLY THERE IS NO INDEX ON LABELS
        // NOW IT IS MIGHT BE TOO SLOW
        for i in 0..label_list.len()  {
            for j in 0..self.nodes_collection.len(){
                if self.nodes_collection[j].labels.contains(&label_list[i]) && !existing_uuids_set.contains(&self.nodes_collection[j].id){
                    existing_uuids_set.insert(self.nodes_collection[j].id);
                    existing_node_refs.push(&self.nodes_collection[j]);
                }
            }
        }

        Ok(existing_node_refs)
    }

    fn get_paths_between_ids(&self, start_id: u32, finish_id: u32) -> Result<Vec<Vec<u32>>, ()>{
        todo!();
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

    pub fn get_nodes_collection_len(&self)-> usize {
        self.nodes_collection.len()
    }

    pub fn get_bonds_collection_len(&self)-> usize {
        self.bonds_collection.len()
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
    graph_data: &GraphCollectionFacade) -> Result<InMemoryGraph, ()> {
    let graphs = graph_data.in_memory_graph_collection.read().unwrap();

    // check if exactly name existst
    for i in 0..graphs.len() {
        if dto.name == graphs[i].name {
            return Err(());
        }
    }

    let graph = InMemoryGraph::new_graph(dto.name);
    Ok(graph)
}