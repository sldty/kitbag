use std::collections::HashMap;
use crate::Address;

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
