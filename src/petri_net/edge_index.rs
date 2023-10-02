use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EdgeIndex{
    edge_index : petgraph::graph::EdgeIndex,
}

impl EdgeIndex{
    pub fn new(edge_index: petgraph::graph::EdgeIndex) -> Self {
        EdgeIndex{
            edge_index,
        }
    }

    pub(super) fn get_edge_index(&self) -> petgraph::graph::EdgeIndex{
        self.edge_index.clone()
    }
}