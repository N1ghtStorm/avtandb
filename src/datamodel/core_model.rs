mod core_model {
    pub struct Graph {
        name: String,
        nodes_collection: Vec<Node>,
        bonds_collection: Vec<Bond>
    }

    pub struct Node {
        id: u32,
    }

    pub struct Bond {
        id: u32,
    }


    impl Graph {
        fn new_graph(name: &str) -> Box<Self> {
            Box::new(Graph {name: String::from(name), 
                     nodes_collection: Vec::new(), 
                     bonds_collection: Vec::new()})
        }

        fn delete_graph(self){
            drop(self);
        }
    }
}