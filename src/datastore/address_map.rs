use std::path::Path;
use crate::{
    handle::{hex, Address},
    datastore::{Storable, DiskMap},
    data::Data
};

/// A content-addressed hash table,
/// that is replicated on disk.
/// Maps the hash of some `Data` (its `Address`)
/// To that data.
pub struct AddressMap {
    /// Maps an `Address` -> `Data`
    contents: DiskMap<Data>
}

impl AddressMap {
    pub fn new(path: &Path) -> Result<AddressMap, String> {
        Ok(AddressMap { contents: DiskMap::new(path)? })
    }

    pub fn contains_address(&self, address: &Address) -> bool {
        self.contents.contains_key(&hex(&address.bytes()))
    }

    pub fn get(&mut self, address: &Address) -> Result<Data, String> {
        self.contents.get(&hex(&address.bytes()))
    }

    pub fn insert(&mut self, data: &Data) -> Result<Address, String> {
        let serialized = Storable::try_to_bytes(data).ok_or("Could not serialize data")?;
        let address = Address::new(&serialized);
        self.contents.insert(&hex(&address.bytes()), data);
        return Ok(address);
    }
}
