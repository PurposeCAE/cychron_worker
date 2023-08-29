use serde::{Serialize, Deserialize};

pub mod action;
pub mod event;

#[derive(Serialize, Deserialize, Debug)]
enum Element{
    Step(Step),
    Transition(Transition),
}

// <<<--- Step --->>>
trait IStep{
    type ActionType: action::IAction;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Step{
    pub actions: Vec<<Self as IStep>::ActionType>,
}

impl IStep for Step{
    type ActionType = action::Action;
}

// <<<--- Transition --->>>
#[derive(Serialize, Deserialize, Debug)]
pub struct Transition{}