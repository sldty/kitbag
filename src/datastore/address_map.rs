use crate::{
    handle::Address,
    datastore::DiskMap,
    data::Data
};

pub struct AddressMap {
    contents: DiskMap<Address, Data>
}

impl AddressMap {
    // new
    // contains_address(Address)
    // get(Address)
    // insert(Content)
}
