use serde::{Serialize, Deserialize};

use crate::{
    agent::Agent,
    page::Page,
    namespace::Namespace,
    handle::Identity
};

#[derive(Clone, Serialize, Deserialize)]
pub enum Storable {
    Agent(Agent),
    Namespace(Namespace),
    Page(Page),
    // TODO: is context it's own thing,
    // or just something that a page has?
    // can the content type of a page change?
    // no. the content type can not change.
    // Content(Content),
}

/// A storable entity:
/// - Has a permanent identity
/// - Is defined within the context of another identity
impl Storable {
    /// An Identity, same across versions
    pub fn identity(&self) -> Identity {
        use Storable::*;
        match self {
            Agent(a)     => a.identity(),
            Namespace(n) => n.identity(),
            Page(p)      => p.identity(),
        }
    }

    // /// basically Clone
    // fn duplicate(&self) -> Box<dyn Storable>;
    /// The enclosing `Identity` as to which this one is relevant.
    /// Essentially works up the context-chain of `Storable`s.
    pub fn context(&self) -> Option<Storable> {
        use Storable::*;
        let context = match self {
            Agent(_)     => return None,
            Namespace(n) => Agent(n.context()),
            Page(p)      => Namespace(p.context()),
        };
        return Some(context);
    }

    /// Find a nested `Identity` inside this one.
    /// Essentially works down the context-chain of `Storable`s.
    pub fn find(&self, identity: &Identity) -> Option<Storable> {
        use Storable::*;
        let found = match self {
            Storable::Agent(a)     => Namespace(a.find(identity)?),
            Storable::Namespace(n) => Page(n.find(identity)?),
            Storable::Page(_)      => return None,
        };
        return Some(found);
    }
}
