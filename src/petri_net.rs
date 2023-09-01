use std::borrow::BorrowMut;

use serde::{Serialize, Deserialize};

use crate::transition::Transition;

use self::edge::Edge;
use self::node::Node;

use super::step::Step;

pub mod node;
mod node_index;
mod edge;


#[derive(Serialize, Deserialize, Debug)]
pub struct PetriNet<'a> {
    petgraph : petgraph::graph::Graph<Step<'a>, Transition<'a>>,

    #[serde(skip)]
    nodes: Vec<Node<'a>>,

    #[serde(skip)]
    edges: Vec<Edge>,

    counter: u32,
}

impl<'a> PetriNet<'a> {
    pub fn new() -> PetriNet<'a> {
        PetriNet{
            petgraph: petgraph::graph::Graph::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
            counter: 0,
        }
    }

    pub fn add_node(&'a mut self, node_data: Step<'a>) -> &mut Node<'a> {

        let node_index = self.petgraph.add_node(node_data);
        let node_data = &mut self.petgraph[node_index];

        let node = Node::new(node_index, node_data);

        self.nodes.push(node);
        // self.nodes.last().unwrap()
        self.nodes.last_mut().unwrap()
    }
    
    pub fn serialize(&self) -> String{
        let serialized_graph = serde_json::to_string_pretty(&self.petgraph).unwrap();
        serialized_graph
    }

    pub fn add_edge(&mut self, parent: &Node, child: &Node, edge_data: Transition<'a>) -> &Edge {
        let child_node_index = child.get_node_index();
        let parent_node_index = parent.get_node_index();

        let edge_index = self.petgraph.add_edge(*parent_node_index, *child_node_index, edge_data);
        self.edges.push(Edge{edge_index});
        self.edges.last().unwrap()
    }

    pub fn get_node_data<'b>(&'a mut self, node_index: &'b petgraph::prelude::NodeIndex) -> &'a mut Step {
        &mut self.petgraph[*node_index]
    }
}