use std::{
    path::PathBuf,
    collections::HashMap,
};

use crate::{
    handle::{Identity, Address},
    diff::Diff,
    storable::Storable, agent::Agent,
};

// TODO: how to make it so the user does not have to have the whole history on-hahd
// but can still work with and modify that which they do have?

// TODO: should branch be storable?
// I think it fits the bill.
/// Represents a single chain of versions across an entire datastore branch.
/// Note that this is not quite the same as a git branch, for instance.
/// Each branch is expected to have one clear owner per node,
/// i.e. all well-formed writes to a branch should succeed.
/// There's some nuance, here, but the idea is for this to work in a local-first manner.
/// Everyone has a local branch that only they can write to,
/// And changes are synced by automatically merging when no conflicts are present.
/// Conflicts are highlighted, and can be manually resolved.
/// Once they've been resolved, changes should automatically propogate across branches.
pub struct Branch {
    /// The owner of the branch. Also can be used to find the root Identity.
    owner: Agent,
    /// All identities and their associated version history.
    histories: HashMap<Identity, History>,
}

impl Branch {
    // pub fn new(owner: Agent) -> Branch {
    //     Branch {
    //         owner,
    //         histories: HashMap::new(),
    //     }
    // }

    // pub fn history(&self, identity: &Identity) -> Option<&History> {
    //     let history = self.identities.get(&identity)?;
    //     return Some(history);
    // }

    pub fn commit(&mut self, identity: &Storable) -> Option<()> {
        // find the right history
        // commit in the history
        todo!()
    }
}

/// Represents a single chain of versions.
/// An append-only datastructure that acts as an ordered map.
/// deltas maps
pub struct History {
    /// A map to any specific version hash.
    addresses: HashMap<Address, usize>,
    /// An ordered list of deltas.
    /// Each delta should be valid in the context of the one before it.
    deltas: Vec<Delta>,
}

impl History {
    pub fn new() -> History {
        // TODO populate with initial data
        History {
            addresses: HashMap::new(),
            deltas:    vec![],
        }
    }

    pub fn version(&self, address: &Address) -> Option<&Delta> {
        let index = self.addresses.get(address)?;
        let delta = &self.deltas[*index];
        return Some(delta);
    }

    pub fn commit(&mut self, next: &Storable) -> Option<()> {
        let head = &self.deltas[self.deltas.len()];
        // TODO: resolve the storable
        // TODO: get the storable's address
        let previous = todo!();
        let address = todo!();
        let delta = Delta::new(previous, next);
        self.deltas.push(delta);
        // TODO: check for conflicts
        self.addresses.insert(address, self.deltas.len());
        todo!()
    }
}

pub struct Delta {
    /// The address of the previous version's content.
    /// Ok(a) means that this is not the root and there is another address.
    /// Err(a) means that this is the root value.
    previous: Result<Address, Storable>,
    /// A diff that can be applied to the previous version to get the next version.
    difference: Diff,
    /// A hash of the content after the diff is applied
    checksum: Address,
}

impl Delta {
    pub fn new(previous: &Storable, next: &Storable) -> Delta {
        // calculate the diffs and addresses
        todo!()
    }
}

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

    // TODO: commit
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
