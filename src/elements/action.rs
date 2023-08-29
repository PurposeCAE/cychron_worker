use serde::{Serialize, Deserialize};

use super::event::{IEvent, Event};

pub trait IAction{
    type EventType: IEvent;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action{
    events: Vec<<Self as IAction>::EventType>,
}
impl IAction for Action {
    type EventType = Event;    
}