use serde::{Deserialize, Serialize};

use self::condition::{CounterCondition, EventDrivenCondition};

pub mod condition;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transition {
    pub(crate) conditions: Condition,
}
impl Transition {
    pub fn new(conditions: Condition) -> Transition {
        Transition { conditions }
    }

    fn get_conditions(&self) -> &Condition {
        &self.conditions
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum Condition {
    EEventDrivenCondition(EventDrivenCondition),
    ECounterCondition(CounterCondition),
}
