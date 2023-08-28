mod graph;
mod node;
mod node_index;
mod edge;

pub fn create_petri_net<N, E>() -> impl IPetriNet<N, E>
    where N: serde::Serialize,
          E: serde::Serialize{
    graph::Graph::new()
}

pub trait IPetriNet<N, E> {
    type NodeType: INode<N, E>;
    type NodeIndexType: INodeIndex;
    type EdgeType: IEdge<N, E>;

    fn add_node(&mut self, node_data: N) -> &Self::NodeType;
    fn serialize(&self) -> String;
}

pub trait INode<N, E>{
    type EdgeType: IEdge<N, E>;

    fn get_parents<T>(&self) -> Vec<&Self::EdgeType>;
    fn get_children<T>(&self) -> Vec<&Self::EdgeType>;
}
pub trait INodeIndex{
    type CounterType: std::ops::Add<Output = Self::CounterType> + Copy + std::fmt::Debug;

    fn new(counter: &Self::CounterType) -> Self;
    fn get_counter(&self) -> &Self::CounterType;
}

pub trait IEdge<N, E>{

}