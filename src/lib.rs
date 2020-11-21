pub mod content;
pub mod data;

pub mod datastore;
pub mod diff;
pub mod handle;

pub use datastore::Datastore;
pub use diff::Diff;
pub use handle::{Tag, Address, Identity, Location};
