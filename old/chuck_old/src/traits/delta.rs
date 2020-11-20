use crate::traits::Storable;

/// Represents the literal difference between two Content.
#[typetag::serde]
pub trait Delta: Storable {
    fn base() -> Self where Self: Sized;
    fn   new<D>(&self, next: Self) -> D where Self: Sized;
    fn apply<D>(&self, diff: &D) -> Self where Self: Sized;
}
