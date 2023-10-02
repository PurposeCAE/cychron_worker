use serde::{Deserialize, Serialize};

use crate::session::IndexMap;
use crate::transition::{Condition, Transition};

use super::step::Step;

mod edge;
mod edge_index;
pub mod node;
mod node_index;

#[derive(Serialize, Deserialize, Debug)]
pub struct PetriNet {
    petgraph: petgraph::graph::Graph<Step, Transition>,

    #[serde(skip)]
    nodes: Vec<self::node_index::NodeIndex>,

    conditions: IndexMap<Condition>,
}

impl PetriNet {
    pub fn new() -> PetriNet {
        PetriNet {
            petgraph: petgraph::graph::Graph::new(),
            nodes: Vec::new(),
            conditions: IndexMap::new(),
        }
    }

    pub fn add_node(&mut self, step: Step) -> node_index::NodeIndex {
        let pet_node_index = self.petgraph.add_node(step);
        let node_index = node_index::NodeIndex::new(pet_node_index);

        self.nodes.push(node_index.clone());

        node_index
    }

    pub fn add_transition(
        &mut self,
        parent_node: node_index::NodeIndex,
        child_node: node_index::NodeIndex,
        transition: Transition,
    ) -> edge_index::EdgeIndex {
        let pet_parent_node_idx = parent_node.get_node_index();
        let pet_child_node_idx = child_node.get_node_index();

        let pet_edge_index =
            self.petgraph
                .add_edge(pet_parent_node_idx, pet_child_node_idx, transition);

        edge_index::EdgeIndex::new(pet_edge_index)
    }

    pub fn serialize(&self) -> String {
        let serialized_graph = serde_json::to_string_pretty(&self.petgraph).unwrap();
        serialized_graph
    }
}
