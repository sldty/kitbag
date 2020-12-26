pub trait Diffable {
    type Diff;

    fn make(prev: &Self, next: &Self) -> Self::Diff;
    fn apply(prev: &Self, diff: &Self::Diff) -> Self;
}
