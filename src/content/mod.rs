pub mod set_diff;
pub mod agent;
pub mod namespace;
pub mod page;

pub use set_diff::SetDiff;
pub use agent::{Agent, AgentDiff};
pub use namespace::{Namespace, NamespaceDiff};
pub use page::{Page, PageDiff};

use serde::{Serialize, Deserialize};

use crate::handle::Location;

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
