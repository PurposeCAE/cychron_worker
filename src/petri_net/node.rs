use crate::step::Step;

#[derive(Debug)]
pub struct Node<'a>{
    node_index : petgraph::graph::NodeIndex,
    pub node_data : &'a mut Step<'a>,
}

impl<'a> Node<'a>{
    pub fn new(node_index : petgraph::prelude::NodeIndex, node_data: &'a mut Step<'a>) -> Node<'a>{
        Node{
            node_index,
            node_data,
        }
    }

    pub fn get_node_index(&self) -> &petgraph::prelude::NodeIndex{
        &self.node_index
    }
}