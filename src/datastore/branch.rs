use std::{
    path::Path,
    collections::HashMap,
};

use crate::{
    handle::Location,
    content::{Contentable, Content},
    datastore::{DiskKV, History},
};

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
    // TODO: identity should take context into account...
    /// All identities and their associated version history.
    histories: DiskKV<History>,
}

impl Branch {
    // TODO: initialize?
    pub fn new(path: &Path) -> Result<Branch, String> {
        Ok(Branch {
            histories: DiskKV::new(path)?,
        })
    }

    pub fn history(&self, location: &Location) -> Option<History> {
        self.histories.load(&location.to_string())
    }

    pub fn update(&mut self, previous: &Content, content: &Content) -> Option<()> {
        let mut history = self.histories.load(&Contentable::location(content).to_string())?;
        history.commit(previous, content);
        // Need to store updated history
        todo!();
        Some(())
    }

    pub fn register(&mut self, content: Content) -> Option<()> {
        let location = Contentable::location(&content);
        let history = History::new(content)?;

        if self.histories.has(&location.to_string()) { return None; }
        else { self.histories.store(&location.to_string(), &history)?; }
        Some(())
    }
}
