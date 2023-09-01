use crate::device::action::Action;
use crate::session::Session;
use crate::transition::condition::EventDrivenCondition;
use crate::{
    step::Step,
    transition::{Condition, Transition},
};

mod device;
mod petri_net;
mod session;
mod step;
mod transition;

fn main() {
    let mut session = Session::new();
    let action1: &mut Action;
    {
        let device1 = session.add_device("device1".to_string());
        action1 = device1.add_new_action("action1".to_string());
        let event1 = action1.add_event("event1".to_string(), 1.0);
    }

    let node1 = session.add_node();
    node1.node_data.actions.push(action1);

    // let transition = Transition{conditions: Condition::EEventDrivenCondition(EventDrivenCondition::new(event1))};


    let mut step1 = Step::new();
    step1.add_action(&action1);
    
    let step2 = Step::new();

    let node1 = session.petri_net.add_node(step1);
    let node2 = session.petri_net.add_node(step2);

    let serialized_petri_net = session.petri_net.serialize();
    println!("{}", serialized_petri_net);
}
