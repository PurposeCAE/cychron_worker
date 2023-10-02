use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Index<T: ?Sized>(u16, PhantomData<T>);

impl<T> Index<T> {
    pub fn new(value: u16) -> Self {
        Index(value, PhantomData)
    }

    pub fn value(&self) -> &u16 {
        &self.0
    }
}