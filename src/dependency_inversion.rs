use crate::{device::event::{Event, IEvent}, petri_net::{IPetriNet, graph::Graph}};

pub trait IDependencyInversion<'a>{
    type EventType: IEvent;
    type PetriNetType: IPetriNet<'a>;
}

pub struct DependencyInversion{}

impl<'a> IDependencyInversion<'a> for DependencyInversion{
    type EventType = Event;
    type PetriNetType = Graph<'a>;
}