/// Represents the address of a Version.
pub enum Address {

}

/// Represents a single version of an entity.
pub enum Version {

}

/// Represents an address that points to an Identity.
pub enum Name {
    Agent(Tag),
}

/// Represents overaching unique identity of a changing entity.
// /// Only valid in the context of a larger Identity,
// /// the largest of which must be proved by cryptographic means.
pub enum Identity {
    Key,
    Agent(Key),
    Namespace(Agent),
    Page(Namespace),
    Branch(Page),
}
