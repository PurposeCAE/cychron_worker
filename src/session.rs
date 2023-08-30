use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{device::Device, petri_net::PetriNet};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session<'a>{
    pub petri_net: PetriNet<'a>,

    devices: HashMap<String, Device>,
}

impl Session<'_>{
    pub fn new() -> Self{
        let petri_net = PetriNet::new();
        Session{
            petri_net,
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device_id: String) -> &mut Device{

        let device = Device::new(device_id.clone());

        self.devices.insert(device_id.clone(), device);

        self.devices.get_mut(&device_id).unwrap()
    }
}