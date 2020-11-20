use serde::{Serialize, Deserialize};
use serde_cbor;
use sha3::{Digest, Sha3_256};
use crate::{
    Permissions,
    Location,
};

/// Is the hash of some content
// #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct Address<T>(String);
//
// impl<T> Address<T> where T: Serialize + DeserializeOwned {
//     pub fn new(content: T) -> Address<T> {
//         serde_cbor::to_vec(content)
//     }
// }

#[derive(Clone, Serialize, Deserialize)]
pub struct Address(Vec<u8>);

impl Address {
    pub fn hash<'de, T>(content: &T) -> (Address, Vec<u8>) where
        T: Serialize + Deserialize<'de>,
    {
        let serialized = serde_cbor::to_vec(content).unwrap();
        let mut hasher = Sha3_256::new();
        hasher.update(serialized.clone());
        let result: Vec<u8> = hasher.finalize().as_slice().to_vec();
        (Address(result), serialized)
    }
}

/// Represents a hierarchical
#[derive(Serialize, Deserialize)]
pub struct Page {
    location:    Box<Location<Page>>,
    permissions: Permissions,
    content:     Box<Address>,
    children:    Vec<Location<Page>>,
    links:       Vec<Location<Page>>,
    backlinks:   Vec<Location<Page>>,
}
