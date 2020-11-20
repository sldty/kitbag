use crate::traits::Content;
use typetag::serde::Serialize;

pub trait Address {
    fn to_bytes(content: &(impl Storable + Serialize)) -> Option<Vec<u8>> where Self: Sized;
    fn hash(bytes: &[u8]) -> Option<Self> where Self: Sized;

    fn address(content: &(impl Storable + Serialize)) -> Option<Self> where Self: Sized {
        Some(Self::hash(&Self::to_bytes(content)?)?)
    }
}

#[typetag::serde]
pub trait Storable: Content where {
    fn addr_ser<A>(&self) -> Option<(A, Vec<u8>)> where Self: Sized + Serialize, A: Address {
        let bytes = A::to_bytes(self)?;
        let address = A::hash(&bytes)?;
        Some((address, bytes))
    }
}
