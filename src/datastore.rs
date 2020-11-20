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
    pub fn update(&mut self, storable: &Storable) -> Option<()> {
        todo!()
    }

    pub fn register(&mut self, storable: &Storable) -> Option<()> {
        todo!()
    }
}

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
    /// The Identity of this branch
    identity: Identity,
    /// All identities and their associated version history.
    histories: HashMap<Identity, History>,
}

impl Branch {
    // TODO: initialize?
    pub fn new(owner: Agent) -> Branch {
        Branch {
            identity: Identity::new(),
            histories: HashMap::new(),
        }
    }

    pub fn update(&mut self, identity: &Storable) -> Option<()> {
        // find the right history
        // commit in the history
        todo!()
    }

    pub fn register(&mut self, identity: &Storable) -> Option<()> {
        todo!()
    }
}

/// Represents a single chain of versions.
/// An append-only datastructure that acts as an ordered map.
/// deltas maps
pub struct History {
    head:   Address,
    deltas: HashMap<Address, Delta>,
}

impl History {
    /// Create a new history.
    pub fn new(initial: Delta) -> History {
        match initial {
            Delta::Base { checksum, .. } => {
                let mut deltas = HashMap::new();
                deltas.insert(checksum, initial);
                return History { head: checksum, deltas };
            },
            Delta::Tip { .. } => unreachable!(),
        }
    }

    /// Commit a delta onto the head history.
    /// Returns None if the delta can not be applied,
    /// Panics if it is passed a base delta, which should be unreachable.
    pub fn commit(&mut self, delta: Delta) -> Option<()> {
        match delta {
            Delta::Tip { previous, difference, checksum } => {
                if previous != self.head { return None; }
                self.deltas.insert(checksum, delta);
                self.head = checksum
            }
            // a history can only have one base,
            // and that base in genned at the start.
            Delta::Base {..} => unreachable!(),
        }
        Some(())
    }

    // pub fn version(&self, address: &Address) -> Option<&Delta> {
    //     let delta = self.deltas.get(address)?;
    //     return Some(delta);
    // }
}

pub enum Delta {
    /// The initial version, to which all changes are applied.
    Base {
        /// The base unit of content itself upon which all deltas are applied.
        base: Storable,
        /// A hash of the base unit of content
        checksum: Address,
    },
    /// A tip applied to either a base (initial version) or another tip
    /// to create a new content
    Tip {
        /// The address of the previous version's content.
        previous: Address,
        /// A diff that can be applied to the previous version to get the next version.
        difference: Diff,
        /// A hash of the content after the diff is applied
        checksum: Address,
    }
}

impl Delta {
    pub fn new(previous: &Storable, next: &Storable) -> Delta {
        // calculate the diffs and addresses
        todo!()
    }
}
