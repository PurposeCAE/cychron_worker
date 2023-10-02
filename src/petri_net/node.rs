#[derive(Debug)]
pub struct Node{
    node_index : petgraph::graph::NodeIndex,
}

impl Node{
    pub fn new(node_index : petgraph::prelude::NodeIndex) -> Node{
        Node{
            node_index,
        }
    }

    pub fn get_node_index(&self) -> &petgraph::prelude::NodeIndex{
        &self.node_index
    }
}