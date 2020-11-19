/// A tag is a unique identifier.
#[derive(Clone)]
pub struct Tag(Vec<u8>);

/// An address is a the immutable handle of a specific version.
#[derive(Clone)]
pub struct Address(Tag);

impl Address {
    fn new(version: &impl Version) -> Address {
        todo!()
    }

    fn tag(&self) -> Tag { self.0.clone() }
}

#[typetag::serde]
pub trait Version {
    fn commit(&self) -> Address {

    }
}

#[typetag::serde]
pub trait Identity {
    fn tag(&self) -> Tag;
    fn context(&self) -> Option<Box<dyn Identity>>;
}

pub trait Storable: Version + Identity {}
impl<T> Storable for T where T: Version + Identity {}
