use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransitionIndex{
    edge_index : petgraph::graph::EdgeIndex,
}

impl TransitionIndex{
    pub fn new(edge_index: petgraph::graph::EdgeIndex) -> Self {
        TransitionIndex{
            edge_index,
        }
    }

    pub(super) fn get_edge_index(&self) -> petgraph::graph::EdgeIndex{
        self.edge_index.clone()
    }
}