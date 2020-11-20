use crate::traits::{
    key,
    Storable
};

/// Represents a wider identity.
/// I.e. an individual Account, an Organization, or so on.
/// Do not assume that a single person has only one identity,
/// or that identities can not consist of groups of people.
/// Identities may be Storable in the future, so they should not have any secrets.
pub trait Identity: Storable {
    fn name(&self) -> String;
    fn public(&self) -> Box<dyn key::Public> where Self: Clone;
}
