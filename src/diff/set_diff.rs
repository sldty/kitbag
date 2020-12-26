use std::{
    hash::Hash,
    collections::HashSet
};

use serde::{Serialize, Deserialize};
use crate::diff::Diffable;

/// The difference between two sets of types.
/// Works by recording the additions and deletions to the two sets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDiff<T> where T: Eq + Hash + Clone {
    added: HashSet<T>,
    removed: HashSet<T>,
}

impl<T> Diffable for HashSet<T> where T: Eq + Hash + Clone {
    type Diff = SetDiff<T>;

    /// Given two sets, this will calculate the additions added
    /// to `next` when compared to `prev`, as well as the deletions,
    /// Which are then used to construct a new `SetDiff`.
    fn make(prev: &Self, next: &Self) -> Self::Diff {
        let added = next.difference(&prev)
            .map(|i| i.clone())
            .collect::<HashSet<T>>();

        let removed = prev.difference(&next)
            .map(|i| i.clone())
            .collect::<HashSet<T>>();

        SetDiff { added, removed }    }

    /// Creates a new set by first removing all entries specified to be removed from a set,
    /// then adding the ones that were not in the original.
    fn apply(prev: &Self, diff: &Self::Diff) -> Self {
        prev.difference(&diff.removed)
            .map(|i| i.clone())
            .collect::<HashSet<T>>()
            .union(&diff.added)
            .map(|i| i.clone())
            .collect::<HashSet<T>>()
    }

}
