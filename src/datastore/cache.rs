use std::path::Path;

use crate::{
    content::Content,
    datastore::{Storable, DiskKV},
    handle::Address
};

/// Wraps a DiskKV to create a cached persistent content-addressed map.
#[derive(Debug)]
pub struct Cache {
    addresses: DiskKV<Content>,
}

impl Cache {
    pub fn new(path: &Path) -> Option<Cache> {
        Some(Cache {
            addresses: DiskKV::new(path)?,
        })
    }

    pub fn has(&mut self, key: &Address) -> bool {
        self.addresses.has(&key.tag().hex())
    }

    pub fn load(&self, key: &Address) -> Option<Content> {
        self.addresses.load(&key.tag().hex())
    }

    pub fn store(&mut self, content: &Content) -> Option<Address> {
        let address = Address::new(&Storable::try_to_bytes(content)?);
        if !self.has(&address) {
            self.addresses.store(&address.tag().hex(), content);
        }
        return Some(address);
    }
}
