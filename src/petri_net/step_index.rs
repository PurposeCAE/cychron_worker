use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StepIndex{
    node_index : petgraph::graph::NodeIndex,
}

impl StepIndex{
    pub fn new(node_index: petgraph::graph::NodeIndex) -> Self {
        StepIndex{
            node_index,
        }
    }

    pub fn get_node_index(&self) -> petgraph::graph::NodeIndex{
        self.node_index.clone()
    }
}