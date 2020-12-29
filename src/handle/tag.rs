use std::time::{SystemTime, UNIX_EPOCH};

use rand::random;
use sha3::{Digest, Sha3_256};
use serde::{Serialize, Deserialize};

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
