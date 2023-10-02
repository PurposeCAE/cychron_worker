use crate::session::index::Index;
use serde::{Deserialize, Serialize};

use super::event::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Action {
    pub(crate) events: Vec<Index<Event>>,
}

impl Action {
    pub fn new() -> Self {
        Action { events: Vec::new() }
    }

    pub fn add_event(&mut self, event_idx: Index<Event>) {
        self.events.push(event_idx);
    }
}
