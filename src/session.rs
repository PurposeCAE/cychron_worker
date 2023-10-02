use serde::{Deserialize, Serialize};

use crate::{
    device::{action::Action, event::Event, Device},
    petri_net::PetriNet,
    session::index::Index,
    session::index_map::IndexMap,
};

pub mod index;
mod index_map;

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    petri_net: PetriNet,

    devices: IndexMap<Device>,
    actions: IndexMap<Action>,
    events: IndexMap<Event>,
}

impl Session {
    pub fn new() -> Self {
        Session {
            petri_net: PetriNet::new(),

            devices: IndexMap::<Device>::new(),
            actions: IndexMap::<Action>::new(),
            events: IndexMap::<Event>::new(),
        }
    }

    pub fn new_device(&mut self) -> Index<Device> {
        self.devices.add(Device::new())
    }

    pub fn add_new_action(&mut self, device_idx: &Index<Device>) -> Index<Action> {
        let device = self.devices.get_mut(&device_idx).unwrap();

        let action = Action::new();
        let action_idx = self.actions.add(action);

        device.add_action(action_idx.clone());

        action_idx
    }

    pub fn add_new_event(&mut self, action_idx: &Index<Action>, rel_time: f64) -> Index<Event> {
        let event = Event::new(rel_time);

        let event_idx = self.events.add(event);

        let action = self.actions.get_mut(&action_idx).unwrap();
        action.add_event(event_idx.clone());

        event_idx
    }

    pub fn get_petri_net(&mut self) -> &mut PetriNet {
        &mut self.petri_net
    }

    pub fn serialize_pretty(&self) -> String {
        let serde_result = serde_json::to_string_pretty(&self);
        let serialized_session = serde_result.unwrap();
        serialized_session
    }
}