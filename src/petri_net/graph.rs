use super::{IPetriNet, INodeIndex};
use super::node::Node;
use super::node_index::NodeIndex;
use super::edge::Edge;

pub struct Graph<N, E>
    where N: serde::Serialize,
          E: serde::Serialize{
    petgraph : petgraph::graph::Graph<N, E>,
    nodes: Vec<<Self as IPetriNet<N, E>>::NodeType>,
    counter: <NodeIndex as INodeIndex>::CounterType,
}

impl<N, E> IPetriNet<N, E> for Graph<N, E>
    where N: serde::Serialize,
          E: serde::Serialize{
    type NodeType = Node;
    type NodeIndexType = NodeIndex;
    type EdgeType = Edge;

    fn add_node(&mut self, node_data: N) -> &Self::NodeType {

        let node_index = self.petgraph.add_node(node_data);

        let node = Node::new(node_index);

        self.nodes.push(node);
        self.nodes.last().unwrap()
    }
    fn serialize(&self) -> String{
        let serialized_graph = serde_json::to_string(&self.petgraph).unwrap();
        serialized_graph
    }
}

impl<N, E> Graph<N, E>
    where N: serde::Serialize,
          E: serde::Serialize{

    pub fn new() -> Graph<N, E>{
        Graph{
            petgraph: petgraph::graph::Graph::new(),
            nodes: Vec::new(),
            counter: 0,
        }
    }
}