use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use crate::{
    handle::{Location, Identity},
    content::{VecDiff, SetDiff, Agent, Page},
};

// TODO: permissions.
/// A `Namespace` holds a collection of `Page`s -
/// `Page`s themselves may hold more complex relationships.
/// But these relationships do not affect the `Identity` of the `Page`.
/// Or the location of the `Page` with respect to the `Namespace`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Namespace {
    // TODO: remove once permissions are implemented?
    /// The owner of the namespace
    agent:        Location,
    pub identity: Identity, // Namespace
    pub title:    String,
    roots:        Vec<Identity>, // Page
    pages:        HashSet<Identity>, // Page
}

impl Namespace {
    /// Given an `Agent`, creates a new namespace, and updates the `Agent`.
    pub fn new(agent: &mut Agent, title: &str) -> Namespace {
        let namespace = Namespace {
            agent:    agent.location(),
            identity: Identity::new(),
            title:    title.to_string(),
            roots:    vec![],
            pages:    HashSet::new(),
        };

        agent.register_namespace(&namespace);
        return namespace;
    }

    // TODO: make register and location functions of `Content`?

    /// Registers a `Page` within the context of this namespace.
    pub fn register_page(&mut self, page: &Page) {
        self.pages.insert(page.identity.clone());
    }

    /// Returns the contextual location of the `Agent`.
    pub fn location(&self) -> Location { self.agent.find(&self.identity) }
}

// TODO: implement vec_diff
/// Represents a difference between two `Namespaces`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceDiff {
    // TODO: change of ownership?
    // agent: Option<Location>,
    title: Option<String>,
    roots: VecDiff<Identity>,
    pages: SetDiff<Identity>,
}

impl NamespaceDiff {
    /// Finds the difference between two `Namespace`s, and creates an `NamespaceDiff`.
    pub fn make(prev: &Namespace, next: &Namespace) -> NamespaceDiff {
        todo!()
    }

    /// Applies this diff to another `Namespace` to create a new `Namespace`.
    pub fn apply(&self, prev: &Namespace) -> Namespace {
        todo!()
    }
}
