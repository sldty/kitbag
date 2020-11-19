use std::{
    path::PathBuf,
    collections::HashMap,
};

use crate::{
    agent::Agent,
    traits::{Address, Storable, Identity},
};

// TODO: should a datastore be storable?
// Probably not.
/// A datastore is a database of two parts:
/// The first part is the content-addressed storage.
/// This is on-disk, with a cache of commonly used items.
/// The second part is a tree of identities.
/// This is built out in-memory, from the relations contained from the content-addressed code.
pub struct Datastore {
    path: PathBuf,
    // TODO: maybe blob type?
    cached_addresses:  HashMap<Address, Vec<u8>>,
    cached_identities: HashMap<Identity, Box<dyn Storable>>,
    local_branch:      Branch,
    cached_branches:   Vec<Branch>,
}

impl Datastore {
    pub fn store(&mut self, storable: &dyn Storable) -> Option<Address> {
        let serialized = rmp_serde::to_vec(storable).ok()?;
        let address    = Address::new(&serialized);
        self.cached_addresses.insert(address.clone(), serialized);
        return Some(address);
    }

    pub fn update(&mut self, storable: &dyn Storable) -> Option<()> /* Option<Delta> */ {
        // get the identity of the storable object
        // find the most current version of that identity on the current branch
        // calculate the delta between that version and this new one
        // calculate the delta address
        // cache & store the delta permanently
        // calculate the content address
        // cache the content
        // update the current version of this identity on the current branch
        // return the delta
        todo!()
    }

    pub fn register(&mut self, storable: &dyn Storable) -> Option<()> {
        // get the identity of the storable object
        // walk the context chain to determine the validity and location of the object
        // calculate the content address
        // cache & store the base version permanently
        // update the current version of this identity on the current branch
        todo!()
    }
}
