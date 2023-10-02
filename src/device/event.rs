use serde::{Deserialize, Serialize};
use std::cmp::Eq;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub rel_time: f64,
}

impl Event {
    pub fn new(rel_time: f64) -> Event {
        Event { rel_time }
    }
}

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.rel_time == other.rel_time
    }
}

impl Hash for Event {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rel_time.to_bits().hash(state);
    }
}
