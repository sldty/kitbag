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
