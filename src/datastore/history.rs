use std::collections::HashMap;
use crate::Address;

/// Represents a series of versions of some Content over time,
/// Within the context of a fork, of course.
pub struct History {
    // The address of the latest version
    head: Address,
    // Maps a content address to a delta that can be used to construct that address
    map:  HashMap<Address, Address>,
}

impl History {
    // new
    // head()
    // contains_address(Address)
    // get(Address)
    // insert(Content)
}
