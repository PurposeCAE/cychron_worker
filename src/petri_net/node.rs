use super::INode;
use super::edge::Edge;

#[derive(Debug)]
pub struct Node{
    node_index : petgraph::graph::NodeIndex,
}

impl Node{
    pub fn new(node_index : petgraph::graph::NodeIndex) -> Node{
        Node{
            node_index
        }
    }
    pub fn get_node_index(&self) -> &petgraph::graph::NodeIndex{
        &self.node_index
    }
}

impl<N, E> INode<N, E> for Node{
    type EdgeType = Edge;
    
    fn get_parents<T>(&self) -> Vec<&Self::EdgeType>{
        Vec::new()
    }
    fn get_children<T>(&self) -> Vec<&Self::EdgeType>{
        Vec::new()
    }
}