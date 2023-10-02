use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeIndex{
    node_index : petgraph::graph::NodeIndex,
}

impl NodeIndex{
    pub fn new(node_index: petgraph::graph::NodeIndex) -> Self {
        NodeIndex{
            node_index,
        }
    }

    pub fn get_node_index(&self) -> petgraph::graph::NodeIndex{
        self.node_index.clone()
    }
}