#![allow(unused)]

use serde::de;
use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};
use session_grpc::session_grpc_server::{SessionGrpc, SessionGrpcServer};
use session_grpc::{AddDeviceCommand, AddDeviceResponse};
pub mod session_grpc {
    tonic::include_proto!("session_grpc");
}

#[derive(Debug)]
pub struct SessionService {
    session: Arc<Mutex<Session>>,
}
impl SessionService {
    pub fn new() -> Self {
        SessionService {
            session: Arc::new(Mutex::new(Session::new())),
        }
    }
}

#[tonic::async_trait]
impl SessionGrpc for SessionService {
    async fn add_device(
        &self,
        request: Request<AddDeviceCommand>,
    ) -> Result<Response<AddDeviceResponse>, Status> {
        println!("Got a request: {:?}", request);
        
        let mut locked_session = self.session.lock().unwrap();
        let device_idx = locked_session.new_device();
        
        let reply = session_grpc::AddDeviceResponse {
            id: *device_idx.value() as u32,
        };

        Ok(Response::new(reply))
    }
}

use crate::{session::Session, transition::{Condition, condition::EventDrivenCondition}};

mod device;
mod petri_net;
mod session;
mod step;
mod transition;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let session_service = SessionService::new();

    Server::builder()
        .add_service(SessionGrpcServer::new(session_service))
        .serve(addr)
        .await?;

    Ok(())
}

/* fn main() {
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
} */
