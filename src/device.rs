use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use self::action::Action;

pub(crate) mod event;
pub(crate) mod action;

#[derive(Serialize, Deserialize, Debug)]
pub struct Device{
    id: String,
    actions: HashMap<String, Action>,
}

impl Device{
    pub fn new(id: String) -> Self{
        Device{
            id,
            actions: HashMap::new(),
        }
    }
    pub fn add_new_action(&mut self, action_id: String) -> &mut Action{

        let action = Action::new(self.id.clone(), action_id.clone());

        self.actions.insert(action_id.clone(), action);

        self.actions.get_mut(&action_id).unwrap()
    }
}