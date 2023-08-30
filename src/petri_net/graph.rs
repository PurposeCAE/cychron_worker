use serde::{Serialize, Deserialize};

use crate::step::Step;

use super::{IPetriNet, INodeIndex};
use super::node::Node;
use super::node_index::NodeIndex;
use super::edge::Edge;
use super::super::transition::Transition;

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph<'a> {
    petgraph : petgraph::graph::Graph<<Self as IPetriNet<'a>>::NodeDataType, <Self as IPetriNet<'a>>::EdgeDataType>,

    #[serde(skip)]
    nodes: Vec<<Self as IPetriNet<'a>>::NodeType>,

    #[serde(skip)]
    edges: Vec<<Self as IPetriNet<'a>>::EdgeType>,

    counter: <NodeIndex as INodeIndex>::CounterType,
}

impl<'a> IPetriNet<'a> for Graph<'_> {
    type NodeType = Node;
    type NodeDataType = Step<'a>;
    type NodeIndexType = NodeIndex;
    type EdgeType = Edge;
    type EdgeDataType = Transition<'a>;

    fn add_node(&mut self, node_data: Self::NodeDataType) -> &Self::NodeType {

        let node_index = self.petgraph.add_node(node_data);

        let node = Node::new(node_index);

        self.nodes.push(node);
        self.nodes.last().unwrap()
    }
    
    fn serialize(&self) -> String{
        let serialized_graph = serde_json::to_string_pretty(&self.petgraph).unwrap();
        serialized_graph
    }

    fn add_edge(&mut self, parent: &Self::NodeType, child: &Self::NodeType, edge_data: Self::EdgeDataType) -> &Self::EdgeType {
        let child_node_index = child.get_node_index();
        let parent_node_index = parent.get_node_index();

        let edge_index = self.petgraph.add_edge(*parent_node_index, *child_node_index, edge_data);
        self.edges.push(Edge{edge_index});
        self.edges.last().unwrap()
    }
}

impl Graph<'_> {

    pub fn new() -> Graph<'static> {
        Graph{
            petgraph: petgraph::graph::Graph::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
            counter: 0,
        }
    }
}