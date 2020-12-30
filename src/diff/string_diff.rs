use crate::diff::{Diffable, VecDiff};

pub struct Chars(String);
pub struct Words(String);

/// Represents a `String`, grouped into lines (by `'\n'`).
/// Useful for when you want to take a courser
/// difference between two `String`s.
pub struct Lines(String);

impl Lines {
    pub fn new(string: String) -> Lines  { Lines(string) }
    pub fn into_inner(self)    -> String { self.0        }

    pub fn lines_inclusive(&self) -> Vec<String> {
        self.0.split("\n").map(|x| x.to_string() + "\n").collect()
    }
}

impl Diffable for Lines {
    type Diff = VecDiff<String>;

    fn make(prev: &Lines, next: &Lines) -> VecDiff<String> {
        Diffable::make(
            &prev.lines_inclusive(),
            &next.lines_inclusive(),
        )
    }

    fn apply(prev: &Lines, diff: &VecDiff<String>) -> Lines {
        Lines::new(Diffable::apply(&prev.lines_inclusive(), &diff).join(""))
    }
}
