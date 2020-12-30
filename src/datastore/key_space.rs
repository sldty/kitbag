use std::path::Path;
use crate::{
    keys::KeyPublic,
    handle::{hex, Identity},
    datastore::{
        Storable,
        DiskMap,
        History
    }
};

/// Represents a space where a public key may write `Content`.
/// Contextualizes Identities.
pub struct KeySpace {
    key_public: KeyPublic,
    /// Maps `Identity` -> `History`
    histories: DiskMap<History>,
}

impl KeySpace {
    // TODO: take a reference to KeyPublic?
    pub fn new(key_public: KeyPublic, path: &Path) -> Result<KeySpace, String> {
        Ok(KeySpace {
            key_public,
            histories: DiskMap::new(path)?,
        })
    }

    pub fn contains_address(&mut self, identity: &Identity) -> bool {
        self.histories.contains_key(&hex(&identity.bytes()))
    }

    pub fn get(&mut self, identity: &Identity) -> Result<History, String> {
        self.histories.get(&hex(&identity.bytes()))
    }

    // pub fn insert(&mut self, content: &Content) -> Result<Identity, String> {
    //     todo!()
    // }

    // new
    // contains_identity(Identity)
    // get(Identity) -> Option<History>
    // insert(Content) -> Option<>
}
