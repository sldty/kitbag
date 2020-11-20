use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::Address;

/// Represents the location and permissions of some content-addressed data.
#[derive(Clone, Serialize, Deserialize)]
pub struct Location<T> {
    address: Address,
    content: Option<T>,
}

impl<T> Location<T> {
    pub fn new<'de>(content: T) -> Location<T> where
        T: Serialize + Deserialize<'de>,
    {
        let (address, _) = Address::hash(&content);
        Location {
            address,
            content: Some(content),
        }
    }

    /// Tries to load the data.
    pub fn resolve(&self) -> T where
        T: Clone,
    {
        if let Some(ref content) = self.content {
            content.to_owned()
        } else {
            // cache, datastore, peers, distributed hash table
            todo!()
        }
    }
}
