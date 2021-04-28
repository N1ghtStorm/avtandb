pub trait Graph {
    fn create_node(&mut self, node: Node) -> Result<(), ()>;
    fn create_bond(&mut self, bond: Bond) -> Result<(), ()>;
} 
/// Main Graph Model
pub struct InMemoryGraph {
    name: String,
    nodes_collection: Vec<Node>,
    bonds_collection: Vec<Bond>
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
    pub fn new_graph(name: &str) -> Self {
        InMemoryGraph {name: String::from(name), 
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
