use serde::{Serialize, Deserialize};

use crate::{
    agent::Agent,
    page::Page,
    namespace::Namespace,
    handle::{Location, Identity}
};

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
    /// An Identity, same across versions
    pub fn location(&self) -> Location {
        use Content::*;
        match self {
            Agent(a)     => a.location(),
            Namespace(n) => n.location(),
            Page(p)      => p.location(),
        }
    }
}
