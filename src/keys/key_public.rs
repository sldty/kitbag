use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyPublic(Vec<u8>);

impl KeyPublic {
    pub fn new(bytes: &[u8]) -> KeyPublic {
        KeyPublic(bytes.to_vec())
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}
