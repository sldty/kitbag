use sha3::{Digest, Sha3_256};
use serde::{Serialize, Deserialize};

// TODO: fixed size?
/// An address is the immutable handle of a specific version of an entity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(Vec<u8>);

impl Address {
    pub fn new(content: &[u8]) -> Address {
        let mut hasher = Sha3_256::new();
        hasher.update(content);
        let result = hasher.finalize();
        Address(result.to_vec())
    }
}
