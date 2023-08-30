use serde::{Serialize, Deserialize};

use crate::dependency_inversion::{IDependencyInversion, DependencyInversion};

use super::ICondition;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventDrivenCondition<'a>{
    pub event_id: String,

    #[serde(skip)]
    pub event: Option<&'a <DependencyInversion as IDependencyInversion<'a>>::EventType>,
}

impl ICondition for EventDrivenCondition<'_>{}

impl EventDrivenCondition<'_> {
    pub fn new(event: & <DependencyInversion as IDependencyInversion>::EventType) -> Self {
        EventDrivenCondition{
            event_id: event.id.clone(),
            event: Some(event),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CounterCondition{
    pub amount_iterations: u32,
}

impl ICondition for CounterCondition {
    
}