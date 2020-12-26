use std::{
    path::{Path, PathBuf},
    collections::HashMap,
};

use crate::{
    content::Content,
    handle::Address
};

#[derive(Debug)]
pub struct KV {
    // The location of the datastore on-disk
    path: PathBuf,
    /// Recently accessed addresses for increased efficiency.
    /// Maps addresses to their serialized representation.
    cache: HashMap<Address, Option<Vec<u8>>>,
}

impl KV {
    pub fn new(path: &Path) -> KV {
        todo!()
    }

    pub fn has(&mut self, address: &Address) -> bool {
        todo!()
    }

    pub fn load(&self, address: &Address) -> Option<Vec<u8>> {
        todo!()
    }

    pub fn store(&mut self, content: &Content) -> Address {
        todo!()
    }
}
