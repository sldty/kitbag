use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH}
};

use rand::random;
use sha3::{Digest, Sha3_256};
use serde::{Serialize, Deserialize};

use crate::{
    content::Content,
    agent::Agent,
    page::Page,
    namespace::Namespace
};

// TODO: set to a fixed size
/// A tag is a unique identifier.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tag(Vec<u8>);

impl Tag {
    pub fn hash(content: &[u8]) -> Tag {
        let mut hasher = Sha3_256::new();
        hasher.update(content);
        let result = hasher.finalize();
        Tag(result.to_vec())
    }

    pub fn generate() -> Tag {
        let mut randstamp = random::<[u8; 32]>().to_vec();
        let mut timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos()
            .to_be_bytes()
            .to_vec();
        randstamp.append(&mut timestamp);
        Tag(randstamp)
    }
}

// /// A blob is a block of binary data.
// #[derive(Clone, Serialize, Deserialize)]
// pub struct Blob(Vec<u8>);

/// An address is the immutable handle of a specific version of an entity.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(Tag);
impl Address {
    pub fn new(content: &[u8]) -> Address { Address(Tag::hash(content)) }
    pub fn tag(&self) -> Tag { self.0.clone() }
}

/// An address is the immutable handle of an entity
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identity(Tag);
impl Identity {
    pub fn new() -> Address { Address(Tag::generate()) }
    pub fn tag(&self) -> Tag { self.0.clone() }
}

// #[derive(Serialize, Deserialize)]
// pub struct Context(HashMap<Identity, Box<dyn Storable>>);
//
// impl Clone for Context {
//     fn clone(&self) -> Context {
//
//     }
// }

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
            Agent(a)     => return None,
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
            Storable::Page(p)      => return None,
        };
        return Some(found);
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Diff {
    Agent(AgentDiff),
    Namespace(NamespaceDiff),
    Page(PageDiff),
}

impl Diff {
    fn make(prev: &Storable, next: &Storable) -> Option<Diff> {
        use Storable::*;
        if prev.identity() != next.identity() { return None; }
        let diff: Diff = match (prev, next) {
            ( Agent(p),     Agent(n)     ) => Diff::Agent(AgentDiff::make(p, n)),
            ( Namespace(p), Namespace(n) ) => Diff::Namespace(NamespaceDiff::make(p, n)),
            ( Page(p),      Page(n)      ) => Diff::Page(PageDiff::make(p, n)),
            _ => return None,
        };
        return Some(diff);
    }

    fn apply(tip: &Storable, diff: &Diff) -> Option<Storable> {
        use Storable::*;
        match (tip, diff) {
            ( Agent(s),     Diff::Agent(d)     ) => d.apply(s),
            ( Namespace(s), Diff::Namespace(d) ) => d.apply(s),
            ( Page(s),      Diff::Page(d)      ) => d.apply(s),
            _ => return None,
        }
        todo!()
    }
}
