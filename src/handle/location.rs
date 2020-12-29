use serde::{Serialize, Deserialize};
use crate::handle::{Address, Identity};

/// A Location is a chain of identities leading to an identity
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location {
    key:         Key,
    identity:    Identity,
    version:     Option<Address>,
}
