use serde::{Deserialize, Serialize};

use crate::{device::event::Event, session::index::Index};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct EventDrivenCondition {
    event_idx: Index<Event>,
}

impl EventDrivenCondition {
    pub fn new(event_idx: Index<Event>) -> EventDrivenCondition {
        EventDrivenCondition { event_idx }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct CounterCondition {
    pub amount_iterations: u32,
}

impl CounterCondition {
    pub fn new(amount_iterations: u32) -> CounterCondition {
        CounterCondition { amount_iterations }
    }
}
