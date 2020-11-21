use std::{
    hash::Hash,
    collections::HashSet
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDiff<T> where T: Eq + Hash + Clone {
    added: HashSet<T>,
    removed: HashSet<T>,
}

impl<T> SetDiff<T> where T: Eq + Hash + Clone {
    pub fn make(prev: &HashSet<T>, next: &HashSet<T>) -> SetDiff<T> {
        let added = next.difference(&prev)
            .map(|i| i.clone())
            .collect::<HashSet<T>>();

        let removed = prev.difference(&next)
            .map(|i| i.clone())
            .collect::<HashSet<T>>();

        SetDiff { added, removed }
    }

    pub fn apply(&self, prev: &HashSet<T>) -> HashSet<T> {
        prev.difference(&self.removed)
            .map(|i| i.clone())
            .collect::<HashSet<T>>()
            .union(&self.added)
            .map(|i| i.clone())
            .collect::<HashSet<T>>()
    }
}
