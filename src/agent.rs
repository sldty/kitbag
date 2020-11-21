use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{
    namespace::Namespace,
    handle::Identity,
};

// TODO: make into trait or enum
// TODO: add keys to agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    display: String,
    identity: Identity, // Agent
    namespaces: HashMap<Identity, Namespace>,
}

impl Agent {
    pub fn new(display: &str) -> Agent {
        Agent {
            display: display.to_string(),
            identity: Identity::new(), // Agent
            namespaces: HashMap::new(),
        }
    }

    pub fn identity(&self) -> Identity { self.identity.clone() }
    // TODO: verify cryptographic keys
    // pub fn context(&self) -> Option<()> { None }

    pub fn find(&self, identity: &Identity) -> Option<Namespace> {
        Some(self.namespaces.get(identity)?.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDiff {}

impl AgentDiff {
    pub fn make(prev: &Agent, next: &Agent) -> AgentDiff {
        todo!()
    }

    pub fn apply(&self, prev: &Agent) -> Agent {
        todo!()
    }
}
