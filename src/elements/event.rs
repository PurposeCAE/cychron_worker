use serde::{Serialize, Deserialize};

pub trait IEvent{}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event{}

impl IEvent for Event {
    
}