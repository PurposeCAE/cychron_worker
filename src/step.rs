use serde::{Serialize, Deserialize};

use crate::device::action::Action;


#[derive(Serialize, Deserialize, Debug)]
pub struct Step<'a>{
    action_ids: Vec<String>,

    #[serde(skip)]
    pub actions: Vec<&'a Action>,
}

impl<'a> Step<'a>{
    pub fn new() -> Step<'a> {
        Step { 
            actions: Vec::new(),
            action_ids: Vec::new(), 
        }
    }
    pub fn add_action(&mut self, action: &'a Action) {
        self.action_ids.push(action.id.clone());
        self.actions.push(action.clone());
    }
}