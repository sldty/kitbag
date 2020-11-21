use serde::{Serialize, Deserialize};

use crate::{
    agent::Agent,
    page::Page,
    namespace::Namespace,
    handle::Identity
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
    pub fn identity(&self) -> Identity {
        use Content::*;
        match self {
            Agent(a)     => a.identity(),
            Namespace(n) => n.identity(),
            Page(p)      => p.identity(),
        }
    }

    // /// basically Clone
    // fn duplicate(&self) -> Box<dyn Content>;
    /// The enclosing `Identity` as to which this one is relevant.
    /// Essentially works up the context-chain of `Content`s.
    pub fn context(&self) -> Option<Content> {
        use Content::*;
        let context = match self {
            Agent(_)     => return None,
            Namespace(n) => Agent(n.context()),
            Page(p)      => Namespace(p.context()),
        };
        return Some(context);
    }

    /// Find a nested `Identity` inside this one.
    /// Essentially works down the context-chain of `Content`s.
    pub fn find(&self, identity: &Identity) -> Option<Content> {
        use Content::*;
        let found = match self {
            Content::Agent(a)     => Namespace(a.find(identity)?),
            Content::Namespace(n) => Page(n.find(identity)?),
            Content::Page(_)      => return None,
        };
        return Some(found);
    }
}
