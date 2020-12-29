use serde::{Serialize, Deserialize};
use crate::handle::Tag;

/// An address is the immutable handle of an entity
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identity(Tag);

impl Identity {
    pub fn new() -> Identity { Identity(Tag::generate()) }
    pub fn tag(&self) -> Tag { self.0.clone() }
}
