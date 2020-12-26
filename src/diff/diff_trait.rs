// TODO: macro to autoderive differences for structs.

/// A structure that can be diffed, i.e. compared with a future structure to produce a diff
/// That can be applied to the original structure the produce the future structure.
pub trait Diffable {
    type Diff;

    fn make(prev: &Self, next: &Self) -> Self::Diff;
    fn apply(prev: &Self, diff: &Self::Diff) -> Self;
}

// Should be used when diffing small fields.
// An indivisible Diff type.
#[derive(Debug, Clone, PartialEq)]
pub struct Atom<T>(T) where T: std::fmt::Debug + Clone + PartialEq;

impl<T> Atom<T> where
    T: std::fmt::Debug + Clone + PartialEq
{
    pub fn new(item: T)     -> Atom<T> { Atom(item) }
    pub fn into_inner(self) -> T       { self.0     }
}

impl<T> Diffable for Atom<T> where
    T: std::fmt::Debug + Clone + PartialEq
{
    type Diff = Option<T>;

    fn make(prev: &Atom<T>, next: &Atom<T>) -> Option<T> {
        if prev != next { Some(next.into_inner()) } else { None }
    }

    fn apply(prev: &Atom<T>, diff: &Option<T>) -> Atom<T> {
        if let Some(next) = diff { Atom::new(next.to_owned()) } else { prev.to_owned() }
    }
}
