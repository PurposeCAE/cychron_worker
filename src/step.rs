use serde::{Serialize, Deserialize};

use crate::{session::Index, device::action::Action};

#[derive(Serialize, Deserialize, Debug)]
pub struct Step{
    actions: Vec<Index<Action>>,
}

impl Step{
    pub fn new() -> Self {
        Step { 
            actions: Vec::new(),
        }
    }

    pub fn add_action(mut self, action_idx: Index<Action>) {
        self.actions.push(action_idx);
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct StepIndex(u32);