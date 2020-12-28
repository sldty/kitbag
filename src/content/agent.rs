use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use crate::{
    content::{
        Contentable,
        Hierarchy,
        HierarchyDiff,
        Namespace,
    },
    handle::{Location, Identity},
    diff::{Diffable, Atom},
};

// TODO: make into trait or enum
// TODO: add keys to agent
/// An `Agent` is a single cryptographically verified identity,
/// That may be controlled by one or more people,
/// Either directly, or through other `Agent`s.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    hierarchy: Hierarchy<(), Namespace>,
    pub identity: Identity,
    pub display:  String,
}

// TODO: add more functionality
impl Agent {
    /// Creates a new `Agent` with a given display name.
    pub fn new(display: &str) -> Agent {
        todo!()
        // Agent {
        //     display: display.to_string(),
        //     identity: Identity::new(), // Agent
        //     namespaces: HashSet::new(),
        // }
    }
}

impl Contentable for Agent {
    fn context(&self)  -> Location {
        Location::root()
    }

    fn identity(&self) -> Identity {
        todo!()
    }

}

/// An `AgentDiff` represents the difference between two agents.
/// Should be used in the context of a `Delta`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDiff {
    hierarchy: HierarchyDiff<(), Namespace>,
    identity:  Option<Identity>,
    display:   Option<String>,
}

impl Diffable for Agent {
    type Diff = AgentDiff;

    /// Finds the difference between two `Agent`s, and creates an `AgentDiff`.
    fn make(prev: &Agent, next: &Agent) -> AgentDiff {
        AgentDiff {
            hierarchy: Diffable::make(&prev.hierarchy, &next.hierarchy),
            identity:  Diffable::make(&Atom::new(&prev.identity), &Atom::new(&next.identity)),
            display:   Diffable::make(&Atom::new(&prev.display),  &Atom::new(&next.display)),
        }
    }

    /// Applies this diff to another `Agent` to create a new `Agent`.
    fn apply(prev: &Agent, diff: &AgentDiff) -> Agent {
        Agent {
            hierarchy: Diffable::apply(&prev.hierarchy, &diff.hierarchy),
            identity:  Diffable::apply(&Atom::new(&prev.identity), &diff.identity).into_inner(),
            display:   Diffable::apply(&Atom::new(&prev.display),  &diff.display).into_inner(),
        }
    }
}
