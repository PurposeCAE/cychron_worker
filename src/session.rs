use std::collections::HashMap;
use crate::petri_net::node::Node;
use serde::{Serialize, Deserialize};

use crate::{device::Device, petri_net::PetriNet, step::{Step, self}};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session<'a>{
    pub petri_net: PetriNet<'a>,

    devices: HashMap<String, Device>,
}

impl<'a> Session<'a>{
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

    pub fn add_node(&'a mut self) ->&mut Node{
        let step = Step::new();

        self.petri_net
            .add_node(step)
    }
}