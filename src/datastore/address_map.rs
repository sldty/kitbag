use crate::{
    handle::Address,
    datastore::DiskMap,
    data::Data
};

/// A content-addressed hash table,
/// that is replicated on disk.
/// Maps the hash of some `Data` (its `Address`)
/// To that data.
pub struct AddressMap {
    contents: DiskMap<Address, Data>
}

impl AddressMap {
    // new
    // contains_address(Address)
    // get(Address)
    // insert(Content)
}
