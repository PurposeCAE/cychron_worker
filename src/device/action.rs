use serde::{Serialize, Deserialize};

use super::event::Event;

#[derive(Serialize, Deserialize, Debug)]
pub struct Action{
    pub(crate) events: Vec<Event>,

    pub device_id: String,

    pub id: String,
}

impl Action {
    pub fn new(device_id: String, id: String) -> Self {
        Action {
            events: Vec::new(),
            device_id,
            id,
        }
    }

    pub fn add_event(&mut self, event_id: String, rel_time: f64) -> &Event {
        let event = Event::new(
            event_id,
            self.device_id.clone(),
            self.id.clone(),
            rel_time,
        );

        self.events.push(event);

        self.events.last().unwrap()
    }
}