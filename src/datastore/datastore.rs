use std::{
    path::{Path, PathBuf},
    collections::HashMap,
};

use crate::{
    handle::Address,
    diff::Diff,
    content::Content,
    datastore::{KV, Delta, Branch},
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
    kv: KV,
}

impl Datastore {
    pub fn new(path: &Path) -> Datastore {
        Datastore {
            local: Branch::new(),
            kv: KV::new(path),
        }
    }

    fn load(&mut self, address: &Address) -> Option<Content> {
        // TODO: schedule on network if not in cache?
        let serialized = self.kv.load(address)?;
        let object = rmp_serde::from_slice(&serialized).ok()?;
        return Some(object);
    }

    fn store(&mut self, content: &Content) -> Option<Address> {
        // TODO: store on disk
        let (address, serialized) = Address::stamp(content)?;
        if !self.kv.has(&address) {
            todo!()
            // self.kv.store(address.clone(), serialized);
        }
        return Some(address);
    }

    // TODO: make more general than deltas
    fn resolve(&mut self, delta: &Delta) -> Option<Content> {
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