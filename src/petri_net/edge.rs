use super::IEdge;

#[derive(Debug)]
pub struct Edge{
    pub(crate) edge_index : petgraph::graph::EdgeIndex,
}

impl<N, E> IEdge<N, E> for Edge{}