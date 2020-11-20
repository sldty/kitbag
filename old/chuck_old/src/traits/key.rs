use typetag;
use crate::traits::Storable;

#[typetag::serde]
pub trait Public: Storable {
    fn encrypt(&self, bytes: Vec<u8>) -> Vec<u8>;
    fn digest(&self) -> Vec<u8>;
    fn verify(&self, digest: Vec<u8>) -> bool;
}

pub trait Private {
    fn decrypt(&self, bytes: Vec<u8>) -> Option<Vec<u8>>;
    fn public(&self) -> Box<dyn Public> where Self: Clone;
    fn verify<P: Public>(&self, public: P) -> bool where Self: Clone;
}
