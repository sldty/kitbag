pub mod data;
pub mod datastore;

pub mod diff;
pub mod handle;
pub mod network;

pub use datastore::Datastore;
pub use diff::Diff;
pub use handle::{Tag, Address, Identity, Location};
pub use network::{KeyPair, encrypt_message, decrypt_message};
