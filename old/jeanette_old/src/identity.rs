use std::hash::{Hash, Hasher};
use crate::{tag, Name};

// TODO: fix the size, ensure uniqueness.
// maybe use a snowflake-like mechanism?
/// A unique identifier for a a Page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    pub name: Name,
    pub tag:  Vec<u8>,
}

impl Identity {
    pub fn new(name: &Name) -> Identity {
        Identity {
            name: name.clone(),
            tag:  tag(),
        }
    }

    pub fn new_same_name(&self) -> Identity {
        Identity::new(&self.name)
    }
}

impl Hash for Identity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
    }
}
