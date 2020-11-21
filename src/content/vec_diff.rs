use serde::{Serialize, Deserialize};

/// Calculates the diff of a vec of items.
/// Uses the Myers diffing algorithm
/// Applies pre-processing steps to increase efficiency,
/// applies post-processing steps to increase readability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VecDiff<T> {
    // todo: T,
}

impl<T> VecDiff<T> {
    // TODO: pre and post processing steps.
    // see: https://github.com/google/diff-match-patch
    // https://neil.fraser.name/writing/diff/myers.pdf
    // and https://neil.fraser.name/writing/diff/ (especially)

    pub fn make(prev: &Vec<T>, next: &Vec<T>) -> VecDiff<T> {
        todo!()
    }

    pub fn apply(&self, prev: &Vec<T>) -> VecDiff<T> {
        todo!()
    }
}
