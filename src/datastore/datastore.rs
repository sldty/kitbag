use std::{
    collections::HashMap,
    path::Path,
};
use crate::{
    keys::KeyPublic,
    datastore::{
        KeySpace,
        AddressMap,
    }
};

/// Represents a key-agnostic Datastore,
/// which is written to disk.
/// It is composed of two parts:
/// 1. A bunch of keyspaces, which contextualizes `Forks`.
/// 2. A single content-addressed `AddressMap` for storing `Data`.
pub struct Datastore {
    keyspaces: HashMap<KeyPublic, KeySpace>,
    address_map: AddressMap,
}

impl Datastore {
    pub fn new(path: &Path) -> Result<Datastore, String> {
        Ok(Datastore {
            keyspaces:   HashMap::new(),
            address_map: AddressMap::new(&path.join("addresses"))?,
        })
    }

    // resolve(Location) -> Content {
    // self.address_map(Location.address)
    // self.keyspaces.get(Location.key_public).history(Location.identity).look_up(address)
    // // TODO
    // }
    // commit(KeyPublic, Content) -> Location

    // add_key(KeyPublic)
}
