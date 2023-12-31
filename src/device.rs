use serde::{Deserialize, Serialize};

use crate::session::index::Index;

use self::action::Action;

pub(crate) mod action;
pub(crate) mod event;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Device {
    actions: Vec<Index<Action>>,
}

impl Device {
    pub fn new() -> Self {
        Device {
            actions: Vec::new(),
        }
    }
    pub fn add_action(&mut self, action_idx: Index<Action>) {
        self.actions.push(action_idx);
    }
}
