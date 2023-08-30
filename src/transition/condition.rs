use serde::{Serialize, Deserialize};

use crate::device::event::Event;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventDrivenCondition<'a>{
    pub event_id: String,

    #[serde(skip)]
    pub event: Option<&'a Event>,
}

impl<'a> EventDrivenCondition<'a> {
    pub fn new(event: &'a Event) -> EventDrivenCondition<'a> {
        EventDrivenCondition {
            event_id: event.id.clone(),
            event: Some(event),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CounterCondition{
    pub amount_iterations: u32,
}