use std::{
    path::PathBuf,
    collections::HashMap,
};

use crate::{
    agent::Agent,
    traits::{Address, Storable, Identity, Diff},
};

// TODO: should a datastore be storable?
// Probably not.
/// A datastore is a database of two parts:
/// The first part is the content-addressed storage.
/// This is on-disk, with a cache of commonly used items.
/// The second part is a tree of identities.
/// This is built out in-memory, from the relations contained from the content-addressed code.
pub struct Datastore {
    path:              PathBuf,
    local_branch:      Branch,
    other_branches:    Vec<Branch>,
    // TODO: maybe blob type?
    cached_addresses:  HashMap<Address, Vec<u8>>,
}

// pub struct Branch {
//     owner: Agent,
// }
//
// impl Branch {
//     pub fn head(&self, identity: &Identity) -> Option<Address> {
//         let versions = self.cached_identities.get(&identity)?;
//         if versions.is_empty() { return None; }
//         return Some(versions[versions.len() - 1].clone().current);
//     }
//
//     pub fn commit(&mut self, identity: &Identity, address: &Address) -> Option<()> {
//         let addresses = self.cached_identities.get_mut(&identity)?;
//         addresses.push(address.clone());
//         Some(())
//     }
// }

#[derive(Clone)]
pub struct Delta {
    /// The previous version
    previous: Address,
    /// A hash of the content after the diff is applied
    current: Address,
    /// A diff that can be applied to the previous version to get the next version.
    difference: Diff,
}

impl Datastore {
    fn load(&self, address: &Address) -> Option<Storable> {
        let serialized = self.cached_addresses.get(address)?;
        let object = rmp_serde::from_slice(serialized).ok()?;
        return Some(object);
    }

    fn store(&mut self, storable: &Storable) -> Option<Address> {
        let serialized = rmp_serde::to_vec(storable).ok()?;
        let address    = Address::new(&serialized);
        // TODO: store address permanently?
        self.cached_addresses.insert(address.clone(), serialized);
        return Some(address);
    }

    pub fn update(&mut self, storable: &Storable) -> Option<Delta> {
        // get the identity of the storable object
        let identity = storable.identity();
        // find the most current version of that identity on the current branch
        let _head = self.load(&self.local_branch.head(&identity)?)?;
        // calculate the delta between that version and this new one
        // let delta: Delta = Delta::make(head, storable);
        // calculate the delta address
        // let delta_address = Version::new(delta);
        // cache & store the delta permanently
        // calculate the content address
        let address = self.store(storable)?;
        // cache the content
        // update the current version of this identity on the current branch
        self.local_branch.commit(&identity, &address)?;
        // return the delta
        todo!()
    }

    pub fn register(&mut self, _storable: &Storable) -> Option<()> {
        // get the identity of the storable object
        // walk the context chain to determine the validity and location of the object
        // calculate the content address
        // cache & store the base version permanently
        // update the current version of this identity on the current branch
        todo!()
    }
}
