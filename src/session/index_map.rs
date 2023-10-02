use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::session::Index;

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexMap<T>
where
    T: Eq,
{
    elements: HashMap<u16, T>,
    idx_counter: u16,
}
impl<T> IndexMap<T>
where
    T: Clone + Eq,
{
    pub fn new() -> Self {
        IndexMap {
            elements: HashMap::new(),
            idx_counter: 0,
        }
    }

    pub fn add(&mut self, element: T) -> Index<T> {
        
        self.idx_counter += 1;
        let elements_index = self.idx_counter;

        self.elements.insert(elements_index, element);

        Index::<T>::new(elements_index)
    }

    pub fn get_mut(&mut self, idx: &Index<T>) -> Option<&mut T> {
        self.elements.get_mut(idx.value())
    }
}