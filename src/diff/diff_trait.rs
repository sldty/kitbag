pub trait Diffable {
    type Diff;

    fn make(&self, next: &Self) -> Self::Diff;
    fn apply(&self, diff: &Self::Diff) -> Self;
}
