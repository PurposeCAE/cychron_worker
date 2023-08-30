

pub struct NodeIndex{
    index: u32,
}

impl NodeIndex{
    fn new(counter: u32) -> Self {
        NodeIndex{
            index: counter,
        }
    }

    fn get_counter(&self) -> u32 {
        self.index
    }
}