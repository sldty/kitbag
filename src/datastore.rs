use std::{
    path::{Path, PathBuf},
    collections::HashMap,
};

use crate::{
    handle::{Identity, Address, Location},
    diff::Diff,
    content::Content,
    content::Agent,
};

// TODO: how to make it so the user does not have to have the whole history on-hahd
// but can still work with and modify that which they do have?

// TODO: how to make it so that user can modify agent without worrying about datastore?
// TODO: implement some form of logarithmic history,
// So changes are cheap and can be made often,
// but will be 'lost' (i.e. combined) over time as to preserve space.
// i.e. when editing, it's useful to be able to undo every character,
// but when looking at a five-year-old document,
// the exact timing of changes to every character is no longer as relevant.

// TODO: should a datastore be content?
// Probably not.
/// A datastore is a database of two parts:
/// The first part is the content-addressed storage.
/// This is on-disk, with a cache of commonly used items.
/// The second part is a tree of identities.
/// This is built out in-memory, from the relations contained from the content-addressed code.
#[derive(Debug)]
pub struct Datastore {
    /// The identity of the local branch.
    local: Branch,
    // /// Map branch identity to branches.
    // branches: HashMap<Identity, Branch>,
    // TODO: build directory hiererchy to aviod rewriting whole datastore.
    /// The write-path of this database.
    path: PathBuf,
    /// Recently accessed addresses for increased efficiency.
    cache: Cache,
}

pub type Cache = HashMap<Address, Vec<u8>>;

impl Datastore {
    pub fn new(path: &Path) -> Datastore {
        Datastore {
            local: Branch::new(),
            path: path.to_path_buf(),
            cache: HashMap::new(),
        }
    }

    fn load(&self, address: &Address) -> Option<Content> {
        // TODO: schedule on network if not in cache?
        let serialized = self.cache.get(address)?;
        let object = rmp_serde::from_slice(serialized).ok()?;
        return Some(object);
    }

    fn store(&mut self, content: &Content) -> Option<Address> {
        // TODO: store on disk
        let (address, serialized) = Address::stamp(content)?;
        if !self.cache.contains_key(&address) {
            self.cache.insert(address.clone(), serialized);
        }
        return Some(address);
    }

    // TODO: make more general than deltas
    fn resolve(&self, delta: &Delta) -> Option<Content> {
        match delta {
            Delta::Base { base, .. } => Some(base.clone()),
            Delta::Tip  { previous, difference, checksum } => {
                // TODO: check datastore cache, then history.
                let prev_content = self.load(&previous)?;
                let next_content = Diff::apply(&prev_content, &difference)?;
                // check the checksum
                if &Address::stamp(&next_content)?.0 != checksum { return None; }
                return Some(next_content)
            }
        }
    }

    // TODO: commit
    // NOTE: just local for now!
    pub fn update(&mut self, content: &Content) -> Option<()> {
        self.store(content)?;
        let history  = self.local.history(&content.location())?;
        let previous = self.load(&history.head)?;
        self.local.update(&previous, content)?;
        return Some(())
    }

    pub fn register(&mut self, content: &Content) -> Option<()> {
        self.store(content)?;
        self.local.register(content.clone())?;
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
#[derive(Debug)]
pub struct Branch {
    /// The Identity of this branch
    // identity: Identity,
    // TODO: identity should take context into account...
    /// All identities and their associated version history.
    histories: HashMap<Location, History>,
}

impl Branch {
    // TODO: initialize?
    pub fn new(/* owner: Agent */) -> Branch {
        Branch {
            // identity: Identity::new(),
            histories: HashMap::new(),
        }
    }

    pub fn history(&self, location: &Location) -> Option<&History> {
        self.histories.get(location)
    }

    pub fn update(&mut self, previous: &Content, content: &Content) -> Option<()> {
        let history = self.histories.get_mut(&content.location())?;
        history.commit(previous, content);
        Some(())
    }

    pub fn register(&mut self, content: Content) -> Option<()> {
        let location = content.location();
        let history = History::new(content)?;

        if self.histories.contains_key(&location) { return None; }
        else { self.histories.insert(location, history)?; }
        Some(())
    }
}

/// Represents a single chain of versions.
#[derive(Debug)]
pub struct History {
    /// The Address of the latest Delta.
    head: Address,
    /// Maps Content addresses to the Delta that is used to make that address.
    deltas: HashMap<Address, Delta>,
}

impl History {
    /// Create a new history.
    fn new(initial: Content) -> Option<History> {
        let address = Address::stamp(&initial)?.0;
        let delta = Delta::base(initial)?;

        let mut deltas = HashMap::new();
        deltas.insert(address.clone(), delta);

        return Some(History { head: address, deltas });
    }

    /// Commit a delta onto the head history.
    /// Returns None if the delta can not be applied,
    /// Panics if it is passed a base delta, which should be unreachable.
    fn commit(&mut self, previous: &Content, next: &Content) -> Option<()> {
        let delta = Delta::make(previous, next)?;

        let address = if let Delta::Tip { previous, checksum, .. } = &delta {
            if previous != &self.head { return None; }
            checksum.clone()
        } else { unreachable!() };

        self.deltas.insert(address.clone(), delta);
        self.head = address;
        return Some(());
    }

    pub fn delta(&self, address: &Address) -> Option<&Delta> {
        let delta = self.deltas.get(address)?;
        return Some(delta);
    }
}

// TODO: make deltas content so that they can be resolved!

#[derive(Debug)]
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
    fn base(initial: Content) -> Option<Delta> {
        let checksum = Address::stamp(&initial)?.0;
        Some(Delta::Base { base: initial, checksum })
    }

    // calculate the diffs and addresses
    fn make(previous: &Content, next: &Content) -> Option<Delta> {
        let prev_addr = Address::stamp(previous)?.0;
        let next_addr = Address::stamp(next)?.0;
        let diff = Diff::make(previous, next)?;

        Some(Delta::Tip {
            previous:   prev_addr,
            difference: diff,
            checksum:   next_addr,
        })
    }
}
