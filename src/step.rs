use serde::{Serialize, Deserialize};

use crate::device::action::Action;


#[derive(Serialize, Deserialize, Debug)]
pub struct Step{
    pub actions: Vec<Action>,
}

impl Step{
    pub fn new() -> Step {
        Step { actions: Vec::new() }
    }
}