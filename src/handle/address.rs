use serde::{Serialize, Deserialize};
use crate::content::Content;
use crate::handle::Tag;

/// An address is the immutable handle of a specific version of an entity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(Tag);

impl Address {
    pub fn new(content: &[u8]) -> Address { Address(Tag::hash(content)) }
    pub fn tag(&self) -> Tag { self.0.clone() }
}
