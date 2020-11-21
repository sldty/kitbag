use std::{
    hash::Hash,
    collections::HashSet
};

use serde::{Serialize, Deserialize};

/// The difference between two sets of types.
/// Works by recording the additions and deletions to the two sets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDiff<T> where T: Eq + Hash + Clone {
    added: HashSet<T>,
    removed: HashSet<T>,
}

impl<T> SetDiff<T> where T: Eq + Hash + Clone {
    /// Given two sets, this will calculate the additions added
    /// to `next` when compared to `prev`, as well as the deletions,
    /// Which are then used to construct a new `SetDiff`.
    pub fn make(prev: &HashSet<T>, next: &HashSet<T>) -> SetDiff<T> {
        let added = next.difference(&prev)
            .map(|i| i.clone())
            .collect::<HashSet<T>>();

        let removed = prev.difference(&next)
            .map(|i| i.clone())
            .collect::<HashSet<T>>();

        SetDiff { added, removed }
    }

    /// Creates a new set by first removing all entries specified to be removed from a set,
    /// then adding the ones that were not in the original.
    pub fn apply(&self, prev: &HashSet<T>) -> HashSet<T> {
        prev.difference(&self.removed)
            .map(|i| i.clone())
            .collect::<HashSet<T>>()
            .union(&self.added)
            .map(|i| i.clone())
            .collect::<HashSet<T>>()
    }
}
