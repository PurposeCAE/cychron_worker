use crate::{step::Step, transition::{Transition, Condition}};
use crate::transition::condition::EventDrivenCondition;
use crate::session::Session;

mod petri_net;
mod step;
mod transition;
mod device;
mod session;

fn main() {
    let mut session = Session::new();
    let device1 = session.add_device("device1".to_string());
    let action1 = device1.add_new_action("action1".to_string());
    let event1 = action1.add_event("event1".to_string(), 1.0);

    let transition = Transition{conditions: Condition::EEventDrivenCondition(EventDrivenCondition::new(event1))};

    let step1 = Step::new();
    let step2 = Step::new();

    let node1 = session.petri_net.add_node(step1);
    let node2 = session.petri_net.add_node(step2);

    let serialized_petri_net = session.petri_net.serialize();
    println!("{}", serialized_petri_net);    
}