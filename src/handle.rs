use std::time::{SystemTime, UNIX_EPOCH};

use rand::random;
use sha3::{Digest, Sha3_256};
use serde::{Serialize, Deserialize};
use crate::storable::Storable;

// TODO: set to a fixed size
/// A tag is a unique identifier.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tag(Vec<u8>);

impl Tag {
    pub fn hash(content: &[u8]) -> Tag {
        let mut hasher = Sha3_256::new();
        hasher.update(content);
        let result = hasher.finalize();
        Tag(result.to_vec())
    }

    pub fn generate() -> Tag {
        let mut randstamp = random::<[u8; 32]>().to_vec();
        let mut timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos()
            .to_be_bytes()
            .to_vec();
        randstamp.append(&mut timestamp);
        Tag(randstamp)
    }
}

// /// A blob is a block of binary data.
// #[derive(Clone, Serialize, Deserialize)]
// pub struct Blob(Vec<u8>);

/// An address is the immutable handle of a specific version of an entity.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(Tag);
impl Address {
    pub fn new(content: &[u8]) -> Address { Address(Tag::hash(content)) }
    pub fn serialize(content: &Storable) -> Vec<u8> { todo!() }
    pub fn tag(&self) -> Tag { self.0.clone() }
}

/// An address is the immutable handle of an entity
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identity(Tag);
impl Identity {
    pub fn new() -> Identity { Identity(Tag::generate()) }
    pub fn tag(&self) -> Tag { self.0.clone() }
}
