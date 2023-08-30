use crate::step;

use super::step::IStep;
use super::transition::ITransition;

pub mod graph;
mod node;
mod node_index;
mod edge;

pub fn create_petri_net() -> impl IPetriNet<'static, NodeDataType = step::Step<'static>> {
    graph::Graph::new()
}

pub trait IPetriNet<'a> {
    type NodeType: INode<Self::NodeDataType, Self::EdgeDataType>;
    type NodeDataType: IStep<'a>;
    type NodeIndexType: INodeIndex;

    type EdgeType: IEdge<Self::NodeDataType, Self::EdgeDataType>;
    type EdgeDataType: ITransition<'a>;

    fn add_node(&mut self, node_data: Self::NodeDataType) -> &Self::NodeType;
    fn add_edge(&mut self, parent: &Self::NodeType, child: &Self::NodeType, edge_data: Self::EdgeDataType) -> &Self::EdgeType;
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