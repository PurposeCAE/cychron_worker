use serde::{Deserialize, Serialize};

use crate::transition::Transition;
use transition_index::TransitionIndex;
use step_index::StepIndex;

use super::step::Step;

mod transition_index;
mod step_index;

#[derive(Serialize, Deserialize, Debug)]
pub struct PetriNet {
    petgraph: petgraph::graph::Graph<Step, Transition>,

    #[serde(skip)]
    nodes: Vec<StepIndex>,
}

impl PetriNet {
    pub fn new() -> PetriNet {
        PetriNet {
            petgraph: petgraph::graph::Graph::new(),
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, step: Step) -> StepIndex {
        let pet_node_index = self.petgraph.add_node(step);
        let node_index = StepIndex::new(pet_node_index);
        
        self.nodes.push(node_index.clone());

        node_index
    }

    pub fn get_mut_step(&mut self, node_index: &StepIndex) -> Option<&mut Step> {
        let pet_node_index = node_index.get_node_index();
        self.petgraph.node_weight_mut(pet_node_index)
    }

    pub fn add_transition(
        &mut self,
        parent_node: StepIndex,
        child_node: StepIndex,
        transition: Transition,
    ) -> TransitionIndex {
        let pet_parent_node_idx = parent_node.get_node_index();
        let pet_child_node_idx = child_node.get_node_index();

        let pet_edge_index =
            self.petgraph
                .add_edge(pet_parent_node_idx, pet_child_node_idx, transition);

        TransitionIndex::new(pet_edge_index)
    }
}