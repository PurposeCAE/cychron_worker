use serde::{Serialize, Deserialize};

use crate::dependency_inversion::{DependencyInversion, IDependencyInversion};

use super::event::IEvent;
use crate::session::Session;

pub trait IAction<'a>{
    type EventType: IEvent;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action<'a>{
    pub(crate) events: Vec<<Self as IAction<'a>>::EventType>,

    pub device_id: String,

    pub id: String,
}
impl<'a> IAction<'a> for Action<'_> {
    type EventType = <DependencyInversion as IDependencyInversion<'a>>::EventType;    
}
impl Action<'_> {
    pub fn new(device_id: String, id: String) -> Self {
        Action {
            events: Vec::new(),
            device_id,
            id,
        }
    }

    pub fn add_event(&mut self, event_id: String, rel_time: f64) -> &<Self as IAction>::EventType {
        let event = <DependencyInversion as IDependencyInversion>::EventType::new(
            event_id,
            self.device_id.clone(),
            self.id.clone(),
            rel_time,
        );

        self.events.push(event);

        self.events.last().unwrap()
    }
}