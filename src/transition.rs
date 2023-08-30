use serde::{Serialize, Deserialize};

use self::condition::{EventDrivenCondition, CounterCondition};

pub mod condition;

pub trait ITransition<'a>{
    fn get_conditions(&self) -> &Condition;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transition<'a>{
    pub(crate) conditions: Condition<'a>
}

impl ITransition<'_> for Transition<'_>{
    fn get_conditions(&self) -> &Condition {
        &self.conditions
    }
}

pub trait ICondition{}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Condition<'a>{
    EEventDrivenCondition(EventDrivenCondition<'a>),
    ECounterCondition(CounterCondition),
}