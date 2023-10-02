use crate::session::Session;

mod device;
mod petri_net;
mod session;
mod step;
mod transition;

fn main() {
    let mut session = Session::new();
    let device_idx = session.new_device();
    let action_idx = session.add_new_action(&device_idx);
    session.add_new_event(&action_idx, 0.0);

    let serialized_session = session.serialize_pretty();
    println!("{}", serialized_session);
}
