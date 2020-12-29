pub mod data;
pub mod datastore;

pub mod diff;
pub mod handle;
pub mod keys;

pub use datastore::Datastore;
pub use diff::Diff;
pub use handle::{Address, Identity, Location};
pub use keys::{KeyPair, KeyPublic, KeySecret};
