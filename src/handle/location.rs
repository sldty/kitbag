use serde::{Serialize, Deserialize};
use crate::{
    keys::KeyPublic,
    handle::{Address, Identity}
};

/// A Location is a specific version of some Content,
/// within the context of a Key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location {
    key_public: KeyPublic,
    identity:   Identity,
    version:    Address,
}

/// A Fork refers to all versions of some Content,
/// within the context of a Key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fork {
    key_public: KeyPublic,
    identity:   Identity,
}
