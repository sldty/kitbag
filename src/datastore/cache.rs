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
    pub fn new(path: &Path) -> Result<Cache, String> {
        Ok(Cache {
            addresses: DiskKV::new(path)?,
        })
    }

    pub fn has(&mut self, key: &Address) -> bool {
        self.addresses.has(&key.tag().hex())
    }

    pub fn load(&self, key: &Address) -> Result<Content, String> {
        self.addresses.load(&key.tag().hex())
        // TODO: check that the address matches the content?
    }

    pub fn store(&mut self, content: &Content) -> Result<Address, String> {
        let address = Address::new(&Storable::try_to_bytes(content).ok_or("Could not serialize content")?);
        if !self.has(&address) {
            self.addresses.store(&address.tag().hex(), content);
        }
        return Ok(address);
    }
}
