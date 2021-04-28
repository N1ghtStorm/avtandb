use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct create_graph_dto {
    pub name: String
}