use crate::{session::Session, transition::{Condition, condition::EventDrivenCondition}};

mod device;
mod petri_net;
mod session;
mod step;
mod transition;

fn main() {
    let mut session = Session::new();
    let device_idx = session.new_device();
    let action_idx = session.add_new_action(&device_idx);
    let eventIndex = session.add_new_event(&action_idx, 0.0);

    let action2_idx = session.add_new_action(&device_idx);
    session.add_new_event(&action2_idx, 0.0);

    let step = step::Step::new();
    let node1_idx = session.get_petri_net().add_node(step);
    let step = session.get_petri_net().get_mut_step(&node1_idx).unwrap();
    step.add_action(action_idx.clone());

    let step = step::Step::new();
    let node2_idx = session.get_petri_net().add_node(step);
    let step = session.get_petri_net().get_mut_step(&node2_idx).unwrap();
    step.add_action(action2_idx.clone());

    let condition = Condition::EEventDrivenCondition(EventDrivenCondition::new(eventIndex.clone()));
    let transition = transition::Transition::new(condition);

    session.get_petri_net().add_transition(node1_idx, node2_idx, transition);

    let serialized_session = session.serialize_pretty();
    println!("{}", serialized_session);
}
