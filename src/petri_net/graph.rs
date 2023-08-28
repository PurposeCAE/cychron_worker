use super::{IPetriNet, INodeIndex};
use super::node::Node;
use super::node_index::NodeIndex;
use super::edge::Edge;

pub struct Graph<N, E> {
    petgraph : petgraph::graph::Graph<N, E>,
    nodes: Vec<<Self as IPetriNet<N, E>>::NodeType>,
    counter: <NodeIndex as INodeIndex>::CounterType,
}

impl<N, E> IPetriNet<N, E> for Graph<N, E>{
    type NodeType = Node;
    type NodeIndexType = NodeIndex;
    type EdgeType = Edge;

    fn add_node(&mut self, node_data: N) -> &Self::NodeType {

        let node_index = self.petgraph.add_node(node_data);

        let node = Node::new(node_index);

        self.nodes.push(node);
        self.nodes.last().unwrap()
    }
}

impl<N, E> Graph<N, E>{
    pub fn new() -> Graph<N, E>{
        Graph{
            petgraph: petgraph::graph::Graph::new(),
            nodes: Vec::new(),
            counter: 0,
        }
    }
}