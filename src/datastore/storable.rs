use serde::{Serialize, de::DeserializeOwned};

// hint: serde -> rmp
pub trait Storable {
    fn try_to_bytes(&self)          -> Option<Vec<u8>>;
    fn try_from_bytes(bytes: &[u8]) -> Option<Box<Self>>;
}

// doh
// impl Storable for Vec<u8> {
//     fn to_bytes(&self)       -> Vec<u8>         { self.clone()       }
//     fn try_from_bytes(&self) -> Option<Vec<u8>> { Some(self.clone()) }
// }

impl<T> Storable for T where T: Serialize + DeserializeOwned {
    fn try_to_bytes(&self) -> Option<Vec<u8>> {
        rmp_serde::to_vec(self).ok()
    }

    fn try_from_bytes(bytes: &[u8]) -> Option<Box<T>> {
        rmp_serde::from_read_ref(bytes).ok()
    }
}
