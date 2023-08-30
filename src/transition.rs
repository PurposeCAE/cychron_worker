use serde::{Serialize, Deserialize};

use self::condition::{EventDrivenCondition, CounterCondition};

pub mod condition;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transition<'a>{
    pub(crate) conditions: Condition<'a>,
}
impl Transition<'_> {
    fn get_conditions(&self) -> &Condition {
        &self.conditions
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Condition<'a>{
    EEventDrivenCondition(EventDrivenCondition<'a>),
    ECounterCondition(CounterCondition),
}