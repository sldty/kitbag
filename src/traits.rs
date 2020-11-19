use serde::{Serialize, Deserialize};
use rmp_serde;

/// A tag is a unique identifier.
#[derive(Clone, Serialize, Deserialize)]
pub struct Tag([u8; 32]);

/// A blob is a block of binary data.
#[derive(Clone, Serialize, Deserialize)]
pub struct Blob(Vec<u8>);

/// An address is the immutable handle of a specific version of an entity.
#[derive(Clone, Serialize, Deserialize)]
pub struct Address(Tag);
impl Address { pub fn tag(&self) -> Tag { self.0.clone() } }

/// An address is the immutable handle of an entity
#[derive(Clone, Serialize, Deserialize)]
pub struct Identity(Tag);
impl Identity { pub fn tag(&self) -> Tag { self.0.clone() } }

/// A storable entity:
/// - Has a permanent identity
/// - Is defined within the context of another identity
#[typetag::serde]
pub trait Storable {
    /// An Identity, same across versions
    fn identity(&self) -> Identity;
    /// The enclosing Identity as to which this one is relevant
    fn context(&self) -> Option<Box<dyn Storable>>;
}
