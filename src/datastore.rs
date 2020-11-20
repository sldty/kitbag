use std::{
    path::PathBuf,
    collections::HashMap,
};

use crate::{
    handle::{Identity, Address},
    diff::Diff,
    content::Content,
    agent::Agent,
};

// TODO: how to make it so the user does not have to have the whole history on-hahd
// but can still work with and modify that which they do have?

// TODO: should a datastore be content?
// Probably not.
/// A datastore is a database of two parts:
/// The first part is the content-addressed storage.
/// This is on-disk, with a cache of commonly used items.
/// The second part is a tree of identities.
/// This is built out in-memory, from the relations contained from the content-addressed code.
pub struct Datastore {
    /// The identity of the local branch.
    local: Branch,
    // /// Map branch identity to branches.
    // branches: HashMap<Identity, Branch>,
    // TODO: build directory hiererchy to aviod rewriting whole datastore.
    /// The write-path of this database.
    path: PathBuf,
    /// Recently accessed addresses for increased efficiency.
    cache: HashMap<Address, Vec<u8>>,
}

// pub type Cache = HashMap<Address, Vec<u8>>;

impl Datastore {
    fn load(&self, address: &Address) -> Option<Content> {
        // TODO: schedule on network if not in cache?
        let serialized = self.cache.get(address)?;
        let object = rmp_serde::from_slice(serialized).ok()?;
        return Some(object);
    }

    fn store(&mut self, content: &Content) -> Option<Address> {
        fn stamp(content: &Content) -> Option<(Address, Vec<u8>)> {
            let serialized = rmp_serde::to_vec(content).ok()?;
            return Some((Address::new(&serialized), serialized));
        }

        let (address, serialized) = stamp(content)?;
        if !self.cache.contains_key(&address) {
            self.cache.insert(address.clone(), serialized);
        }
        return Some(address);
    }

    // TODO: commit
    // NOTE: just local for now!
    pub fn update(&mut self, content: &Content) -> Option<()> {
        // let address = self.store(content)?;
        let branch = &mut self.local;
        branch.update(self, content)?;
        todo!()
    }

    pub fn register(&mut self, content: &Content) -> Option<()> {
        todo!()
    }
}

// TODO: should branch be content?
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
    // identity: Identity,
    // TODO: identity should take context into account...
    /// All identities and their associated version history.
    histories: HashMap<Identity, History>,
}

impl Branch {
    // TODO: initialize?
    pub fn new(/* owner: Agent */) -> Branch {
        Branch {
            // identity: Identity::new(),
            histories: HashMap::new(),
        }
    }

    pub fn history(&self, content: &Content) -> Option<&History> {
        self.histories.get(&content.identity())
    }

    pub fn update(&mut self, datastore: &mut Datastore, content: &Content) -> Option<()> {
        let history = self.histories.get_mut(&content.identity())?;
        history.commit(datastore, content);
        Some(())
    }

    pub fn register(&mut self, datastore: &mut Datastore, content: Content) -> Option<()> {
        let identity = content.identity();
        let history = History::new(datastore, content)?;

        if self.histories.contains_key(&identity) { return None; }
        else { self.histories.insert(identity, history)?; }
        Some(())
    }
}

/// Represents a single chain of versions.
/// An append-only datastructure that acts as an ordered map.
/// deltas maps
pub struct History {
    /// The Address of the latest Delta.
    head: Address,
    /// Maps Content addresses to the Delta that is used to make that address.
    deltas: HashMap<Address, Delta>,
}

impl History {
    /// Create a new history.
    fn new(datastore: &mut Datastore, initial: Content) -> Option<History> {
        let address = datastore.store(&initial)?;
        let delta = Delta::base(datastore, initial)?;

        let mut deltas = HashMap::new();
        deltas.insert(address.clone(), delta);

        return Some(History { head: address, deltas });
    }

    /// Commit a delta onto the head history.
    /// Returns None if the delta can not be applied,
    /// Panics if it is passed a base delta, which should be unreachable.
    fn commit(&mut self, datastore: &mut Datastore, content: &Content) -> Option<()> {
        let previous = self.delta(&self.head)?.resolve(datastore)?;
        let delta = Delta::new(datastore, &previous, content)?;
        let address = datastore.store(content)?;

        self.deltas.insert(address.clone(), delta);
        self.head = address;
        Some(())
    }

    pub fn delta(&self, address: &Address) -> Option<&Delta> {
        let delta = self.deltas.get(address)?;
        return Some(delta);
    }
}

// TODO: make deltas content so that they can be resolved!

pub enum Delta {
    /// The initial version, to which all changes are applied.
    Base {
        /// The base unit of content itself upon which all deltas are applied.
        base: Content,
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

/// Generated deltas must be stored in the datastore!
impl Delta {
    fn base(datastore: &mut Datastore, initial: Content) -> Option<Delta> {
        let checksum = datastore.store(&initial)?;
        Some(Delta::Base { base: initial, checksum })
    }

    // calculate the diffs and addresses
    fn new(datastore: &mut Datastore, previous: &Content, next: &Content) -> Option<Delta> {
        let prev_addr = datastore.store(previous)?;
        let next_addr = datastore.store(next)?;
        let diff = Diff::make(previous, next)?;

        Some(Delta::Tip {
            previous:   prev_addr,
            difference: diff,
            checksum:   next_addr,
        })
    }

    fn resolve(&self, datastore: &mut Datastore) -> Option<Content> {
        match self {
            Delta::Base { base, .. } => Some(base.clone()),
            Delta::Tip  { previous, difference, checksum } => {
                // TODO: check datastore cache, then history.
                let prev_content = datastore.load(previous)?;
                let next_content = Diff::apply(&prev_content, difference)?;
                // check the checksum
                if &datastore.store(&next_content)? != checksum { return None; }
                return Some(next_content)
            }
        }
    }
}
