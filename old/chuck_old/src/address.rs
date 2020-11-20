// use serde::{Serialize, Deserialize};
use rmp_serde;
use sha3::{Digest, Sha3_256};
use crate::traits::{Address, Storable};

pub struct MpSha3(Vec<u8>);

impl Address for MpSha3 {
    fn to_bytes(content: &(impl Storable + typetag::serde::Serialize)) -> Option<Vec<u8>> where Self: Sized {
        rmp_serde::to_vec(content).ok()
    }

    fn hash(bytes: &[u8]) -> Option<Self> where Self: Sized {
        let mut hasher = Sha3_256::new();
        hasher.update(bytes.clone());
        let result: Vec<u8> = hasher.finalize().as_slice().to_vec();
        Some(MpSha3(result))
    }
}
