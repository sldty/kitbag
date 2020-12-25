use std::collections::HashMap;

use crate::{
    handle::Location,
    content::Content,
    datastore::History,
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
