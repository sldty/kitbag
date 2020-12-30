use crate::{
    keys::KeyPublic,
    handle::Identity,
    datastore::{
        DiskMap,
        History
    }
};

/// Represents a space where a public key may write `Content`.
/// Contextualizes Identities.
pub struct KeySpace {
    key_public: KeyPublic,
    delta_maps: DiskMap<Identity, History>,
}

impl KeySpace {
    // new
    // contains_identity(Identity)
    // get(Identity) -> Option<History>
    // insert(Content) -> Option<>
}
