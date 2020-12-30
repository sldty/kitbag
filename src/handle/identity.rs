use std::time::{SystemTime, UNIX_EPOCH};
use rand::random;
use serde::{Serialize, Deserialize};

// TODO: fixed size?
/// An `Identity` is a unique identifier assigned to some `Content`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identity(Vec<u8>);

impl Identity {
    pub fn new() -> Identity {
        let mut timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos()
            .to_be_bytes()
            .to_vec();
        let mut randstamp = random::<[u8; 16]>().to_vec();
        timestamp.append(&mut randstamp);
        Identity(timestamp)
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}
