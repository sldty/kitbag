use serde::{Serialize, Deserialize};
use crate::{
    Permissions,
    Location
};

pub trait Diff<D> {
    fn  base() -> Self where Self: Sized;
    fn   new(prev: &Self, next: &Self) -> D where Self: Sized;
    fn apply(prev: &Self, diff: &D) -> Self where Self: Sized;
}

/// Represents the difference between two versions of a file.
#[derive(Clone, Serialize, Deserialize)]
pub struct Delta<C, D> where
    C: Diff<D> + Clone,
    D: Clone,
{
    /// None means diff is base Diff.
    previous: Option<Box<Location<Delta<C, D>>>>,
    /// The difference between this and the previous verison.
    diff:     Box<D>,
    /// All the diffs applied on top of one another,
    /// acts as a cache layer.
    content:  Option<C>,
}

impl<C, D> Delta<C, D> where
    C: Diff<D> + Clone,
    D: Clone,
{
    pub fn new(content: C) -> Self {
        Delta {
            previous: None,
            diff: Box::new(Diff::new(&Diff::base(), &content)),
            content: Some(content),
        }
    }

    pub fn extend(
        previous: Box<Location<Delta<C, D>>>,
        current: C,
        permissions: Permissions,
    ) -> Self {
        let diff: D = Diff::new(&previous.resolve().content(), &current);
        Delta {
            previous: Some(previous),
            diff: Box::new(diff),
            content: Some(current),
        }
    }

    // TODO: reference?
    pub fn content(&mut self) -> C {
        if let Some(ref content) = self.content {
            content.to_owned()
        } else if let Some(ref mut previous) = self.previous {
            let previous = previous.resolve().content();
            let content   = Diff::apply(&previous, &self.diff);
            self.content  = Some(content.clone());
            content
        } else {
            let content:  C = Diff::apply(&Diff::base(), &self.diff);
            self.content = Some(content.clone());
            content
        }
    }
}

// Example:
//
// #[derive(Clone)]
// struct Number(f64);
// #[derive(Clone)]
// struct NumberDiff(f64);
//
// impl Diff<NumberDiff> for Number {
//     fn empty() -> Number {
//         Number(0.0)
//     }
//
//     fn new(prev: Number, next: Number) -> NumberDiff {
//         let Number(prev) = prev;
//         let Number(next) = next;
//         NumberDiff(next - prev)
//     }
//
//     fn apply(prev: Number, diff: NumberDiff) -> Number {
//         let Number(prev) = prev;
//         let NumberDiff(diff) = diff;
//         Number(prev + diff)
//     }
// }
