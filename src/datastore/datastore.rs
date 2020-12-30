use crate::datastore::{
    KeySpace,
    AddressMap,
};

pub struct Datastore {
    keyspaces: HashMap<KeyPublic, KeySpace>,
    address_map: AddressMap,
}

impl Datastore {
    // new
    // resolve(Location) -> Content {
    // self.address_map(Location.address)
    // self.keyspaces.get(Location.key_public).history(Location.identity).look_up(address)
    // // TODO
    // }
    // commit(KeyPublic, Content) -> Location
}
