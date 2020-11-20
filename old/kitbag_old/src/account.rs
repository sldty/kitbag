use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
// TODO: trait

/// Represents a single identity,
/// i.e. a set of keys and associated metadata,
/// for a user.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    display_name: String,
    public_key: Vec<u8>,
}

impl Hash for Account {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.public_key);
    }
}

// pub struct Organization {
//     display_name: String,
//     can_control: HashMap<...>
//     // digest: X,
//     // public_key: X,
//     // private_key: Option<X>,
// }
