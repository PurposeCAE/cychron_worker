use petri_net::IPetriNet;
use serde::{Serialize, Deserialize};
mod petri_net;

fn main() {
    let mut petri_net = petri_net::create_petri_net::<Element, Binder>();
    let node = petri_net.add_node(Element{name: "test".to_string()});

    let serialized_petri_net = petri_net.serialize();
    println!("{}", serialized_petri_net);

}

#[derive(Serialize, Deserialize, Debug)]
struct Element{
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Binder{

}