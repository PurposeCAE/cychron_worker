use super::INodeIndex;

pub struct NodeIndex{
    index: <Self as INodeIndex>::CounterType,
}

impl INodeIndex for NodeIndex{
    type CounterType = u32;

    fn new(counter: &Self::CounterType) -> Self {
        NodeIndex{
            index: *counter,
        }
    }

    fn get_counter(&self) -> &Self::CounterType {
        &self.index
    }
}