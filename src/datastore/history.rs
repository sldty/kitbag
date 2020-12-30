use std::{
    path::Path,
    collections::HashMap,
};
use serde::{Serialize, Deserialize};
use crate::Address;

/// Represents a series of versions of some Content over time,
/// Within the context of a fork, of course.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    // The address of the latest version
    head: Address,
    // Maps a content `Address` -> delta `Address`
    map:  HashMap<Address, Address>,
}

impl History {
    pub fn new() -> Result<History, String> {
        Ok(History {
            head: todo!(),
            map:  HashMap::new(),
        })
    }

    pub fn head(&self) -> Address { self.head.clone() }

    pub fn contains_address(&self, address: &Address) -> bool {
        self.map.contains_key(address)
    }


    // head()
    // contains_address(Address)
    // get(Address)
    // insert(Content)
}
