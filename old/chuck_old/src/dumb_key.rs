use serde::{Serialize, Deserialize};
use crate::traits::{
    key,
    Storable,
    Content,
};

#[derive(Serialize, Deserialize)]
pub struct DumbPublic();

#[typetag::serde] impl Content for DumbPublic {}
#[typetag::serde] impl Storable for DumbPublic {}

#[typetag::serde]
impl key::Public for DumbPublic {
    fn encrypt(&self, bytes: Vec<u8>) -> Vec<u8> { bytes }
    fn digest(&self) -> Vec<u8> { vec![] }
    fn verify(&self, _digest: Vec<u8>) -> bool { true }
}

pub struct DumbPrivate();

impl key::Private for DumbPrivate {
    fn decrypt(&self, bytes: Vec<u8>) -> Option<Vec<u8>> { Some(bytes) }
    fn public(&self) -> Box<dyn key::Public> where Self: Sized { Box::new(DumbPublic()) }
    fn verify<P: key::Public>(&self, _public: P) -> bool where Self: Sized { true }
}
