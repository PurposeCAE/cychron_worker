use serde::{Serialize, Deserialize};

pub trait IEvent{
    fn new(id: String, action_id: String, device_id: String, rel_time: f64) -> Self where Self: Sized;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event{
    pub id: String,
    pub action_id: String,
    pub device_id: String,
    pub rel_time: f64,
}

impl IEvent for Event {
    fn new(id: String, action_id: String, device_id: String, rel_time: f64) -> Self where Self: Sized {
        Event{
            id,
            action_id,
            device_id,
            rel_time,
        }
    }
}