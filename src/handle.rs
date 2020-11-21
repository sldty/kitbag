use std::time::{SystemTime, UNIX_EPOCH};

use rand::random;
use sha3::{Digest, Sha3_256};
use serde::{Serialize, Deserialize};
use crate::content::Content;

// TODO: set to a fixed size
/// A tag is a unique identifier.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tag(Vec<u8>);

impl Tag {
    pub fn hex(&self) -> String {
        self.0.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }

    pub fn hash(content: &[u8]) -> Tag {
        let mut hasher = Sha3_256::new();
        hasher.update(content);
        let result = hasher.finalize();
        Tag(result.to_vec())
    }

    pub fn generate() -> Tag {
        let mut timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos()
            .to_be_bytes()
            .to_vec();
        let mut randstamp = random::<[u8; 16]>().to_vec();
        timestamp.append(&mut randstamp);
        Tag(timestamp)
    }
}

impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("Tag")
            .field(&self.hex())
            .finish()
    }
}

/// A blob is a block of binary data.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Blob(Tag);

impl Blob {
    pub fn new(content: &Content) -> Option<Blob> {
        let serialized = rmp_serde::to_vec(content).ok()?;
        return Some(Blob(Tag(serialized)));
    }
}

/// An address is the immutable handle of a specific version of an entity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(Tag);
impl Address {
    pub fn new(content: &[u8]) -> Address { Address(Tag::hash(content)) }
    pub fn tag(&self) -> Tag { self.0.clone() }

    pub fn stamp(content: &Content) -> Option<(Address, Vec<u8>)> {
        let serialized = rmp_serde::to_vec(content).ok()?;
        return Some((Address::new(&serialized), serialized));
    }
}

/// An address is the immutable handle of an entity
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identity(Tag);
impl Identity {
    pub fn new() -> Identity { Identity(Tag::generate()) }
    pub fn tag(&self) -> Tag { self.0.clone() }
}

// TODO: restrict type?
// only locations for:
// - Agent
// - Namespace
// - Page
// are possible, and:
// Agent: 1 Id.
// Namespace: 2 Id.s
// Page 3 Id.s

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location(Vec<Identity>);

impl Location {
    pub fn root() -> Location { Location(vec![]) }

    pub fn find(&self, identity: &Identity) -> Location {
        let mut chain = self.0.clone();
        chain.push(identity.clone());
        return Location(chain);
    }
}
