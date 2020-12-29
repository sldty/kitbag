use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use crate::{
    handle::{Location, Identity},
    diff::{VecDiff, SetDiff, Diffable},
    content::{Contentable, Hierarchy, HierarchyDiff, Agent, Page},
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
    pub hierarchy: Hierarchy<Agent, Page>,
    pub identity: Identity,
    pub title: String,
    pub roots: Vec<Identity>,
}

impl Namespace {
    /// Given an `Agent`, creates a new namespace, and updates the `Agent`.
    pub fn new(agent: &mut Agent, title: &str) -> Namespace {
        let namespace = Namespace {
            hierarchy: Hierarchy::new(agent),
            identity:  Identity::new(),
            title:     title.to_string(),
            roots:     vec![],
        };

        agent.hierarchy.register(&namespace);
        return namespace;
    }
}

impl Contentable for Namespace {
    fn context(&self)  -> Location { self.hierarchy.parent.clone()   }
    fn identity(&self) -> Identity { self.hierarchy.identity.clone() }
}

// TODO: implement vec_diff
/// Represents a difference between two `Namespaces`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceDiff {
    pub hierarchy: HierarchyDiff<Agent, Page>,
    pub identity:  Option<Identity>,
    pub title:     Option<String>,
    pub roots:     VecDiff<Identity>,
}

impl Diffable for Namespace {
    type Diff = NamespaceDiff;

    /// Finds the difference between two `Namespaces`s, and creates an `Namespaces`.
    fn make(prev: &Namespace, next: &Namespace) -> NamespaceDiff {
        todo!()
    }

    /// Applies this diff to another `Agent` to create a new `Agent`.
    fn apply(prev: &Namespace, diff: &NamespaceDiff) -> Namespace {
        todo!()
    }
}
