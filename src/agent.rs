use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use crate::{
    namespace::Namespace,
    handle::{Location, Identity}, set_diff::SetDiff,
};

// TODO: make into trait or enum
// TODO: add keys to agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub display:  String,
    pub identity: Identity, // Agent
    namespaces:   HashSet<Identity>,
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
    namespaces_diff: SetDiff<Identity>,
}

impl AgentDiff {
    pub fn make(prev: &Agent, next: &Agent) -> AgentDiff {
        let display = if prev.display != next.display { Some(next.display.clone()) }
            else { None };

        let namespaces_diff = SetDiff::make(&prev.namespaces, &next.namespaces);
        AgentDiff { display, namespaces_diff }
    }

    pub fn apply(&self, prev: &Agent) -> Agent {
        let display = if let Some(new) = &self.display { new.to_string() }
            else { prev.display.to_string() };

        let identity = prev.identity.clone();
        let namespaces = self.namespaces_diff.apply(&prev.namespaces);
        Agent { display, identity, namespaces }
    }
}
