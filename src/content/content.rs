use serde::{Serialize, Deserialize};
use crate::handle::Location;

use crate::content::{
    Agent,
    Namespace,
    Page,
};

/// A piece of `Content` is a user-facing data that
/// forms a publically visible relational hierarchy:
/// - An `Agent` has any number of `Namespace`s,
/// - A `Namespace` has any number of `Pages`
/// - `Pages` are the base unit, but can be hierarchically linked to and organized.
/// Note that all `Content` is closely linked to `Datastore`
/// All new `Content` must be `datastore.register(...)`d
/// All modified `Content` must be `datastore.update(...)`d
/// All functions that modify `Content` should take a mutable reference
/// to any other content affected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Content {
    Agent(Agent),
    Namespace(Namespace),
    Page(Page),
    // TODO: is context it's own thing,
    // or just something that a page has?
    // can the content type of a page change?
    // no. the content type can not change.
    // Content(Content),
}

/// A content entity:
/// - Has a permanent identity
/// - Is defined within the context of another identity
impl Content {
    // TODO: remove?
    /// An Identity, same across versions.
    pub fn location(&self) -> Location {
        use Content::*;
        match self {
            Agent(a)     => a.location(),
            Namespace(n) => n.location(),
            Page(p)      => p.location(),
        }
    }
}
