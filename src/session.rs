use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{device::Device, dependency_inversion::{DependencyInversion, IDependencyInversion}, petri_net::IPetriNet};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session<'a>{
    pub petri_net: <DependencyInversion as IDependencyInversion<'a>>::PetriNetType,

    devices: HashMap<String, Device<'a>>,
}

impl Session<'_>{
    pub fn new() -> Self{
        let petri_net = <DependencyInversion as IDependencyInversion>::PetriNetType::new();
        Session{
            petri_net,
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device_id: String) -> &Device{
        let device = Device::new(device_id);
        self.devices.insert(device_id.clone(), device);
        self.devices.get(&device_id).unwrap()
    }
}