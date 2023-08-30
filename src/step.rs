use serde::{Serialize, Deserialize};

use crate::{petri_net::{IPetriNet, self}, device::action};

pub trait IStep<'a>{
    type ActionType: action::IAction<'a>;
    fn new() -> <petri_net::graph::Graph<'static> as IPetriNet<'static>>::NodeDataType;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Step<'a>{
    pub actions: Vec<<Self as IStep<'a>>::ActionType>,
}

impl<'a> IStep<'a> for Step<'_>{
    type ActionType = action::Action<'a>;

    fn new() -> <petri_net::graph::Graph<'static> as IPetriNet<'static>>::NodeDataType {
        Step { actions: Vec::new() }
    }
}