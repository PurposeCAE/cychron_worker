use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::session::Session;
use self::action::Action;

pub(crate) mod event;
pub(crate) mod action;

#[derive(Serialize, Deserialize, Debug)]
pub struct Device<'a>{
    id: String,
    actions: HashMap<String, Action<'a>>,
}

impl Device<'_>{
    pub fn new(id: String) -> Self{
        Device{
            id,
            actions: HashMap::new(),
        }
    }
    pub fn add_new_action(&mut self, action_id: String) -> &Action{

        let action = Action::new(self.id.clone(), action_id);

        self.actions.insert(action_id.clone(), action);

        self.actions.get(&action_id).unwrap()
    }
}