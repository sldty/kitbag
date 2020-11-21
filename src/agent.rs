use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use crate::{
    namespace::Namespace,
    handle::{Location, Identity},
};

// TODO: make into trait or enum
// TODO: add keys to agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub display: String,
    pub identity: Identity, // Agent
    namespaces: HashSet<Identity>,
}

impl Agent {
    pub fn new(display: &str) -> Agent {
        Agent {
            display: display.to_string(),
            identity: Identity::new(), // Agent
            namespaces: HashSet::new(),
        }
    }

    pub fn register_namespace(&mut self, namespace: &Namespace) {
        self.namespaces.insert(namespace.identity.clone());
    }

    pub fn location(&self) -> Location { Location::root().find(&self.identity) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDiff {
    display: Option<String>,
    added: HashSet<Identity>,
    removed: HashSet<Identity>,
}

impl AgentDiff {
    pub fn make(prev: &Agent, next: &Agent) -> AgentDiff {
        let display = if prev.display != next.display { Some(next.display.clone()) }
            else { None };

        let added = next.namespaces
            .difference(&prev.namespaces)
            .map(|i| i.clone())
            .collect::<HashSet<Identity>>();

        let removed = prev.namespaces
            .difference(&next.namespaces)
            .map(|i| i.clone())
            .collect::<HashSet<Identity>>();

        AgentDiff { display, added, removed }
    }

    pub fn apply(&self, prev: &Agent) -> Agent {
        todo!()
    }
}
