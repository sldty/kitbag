pub mod set_diff;
pub mod vec_diff;
pub mod string_diff;
pub mod diffable;

pub use set_diff::SetDiff;
pub use vec_diff::VecDiff;
pub use string_diff::Lines;

pub use diffable::{Diffable, Atom};
