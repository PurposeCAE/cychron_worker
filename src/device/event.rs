use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event{
    pub id: String,
    pub action_id: String,
    pub device_id: String,
    pub rel_time: f64,
}

impl Event {
    pub fn new(id: String, action_id: String, device_id: String, rel_time: f64) -> Event {
        Event{
            id,
            action_id,
            device_id,
            rel_time,
        }
    }
}