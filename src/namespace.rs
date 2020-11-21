use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{
    agent::Agent,
    page::Page,
    handle::Identity
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Namespace {
    /// The owner of the namespace
    agent:    Agent,
    identity: Identity, // Namespace
    title:    String,
    root:     Identity, // Page
    pages:    HashMap<Identity, Page>,
}

impl Namespace {
    pub fn identity(&self) -> Identity { self.identity.clone() }

    pub fn context(&self) -> Agent {
        self.agent.clone()
    }

    pub fn find(&self, identity: &Identity) -> Option<Page> {
        Some(self.pages.get(identity)?.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceDiff {}

impl NamespaceDiff {
    pub fn make(prev: &Namespace, next: &Namespace) -> NamespaceDiff {
        todo!()
    }

    pub fn apply(&self, prev: &Namespace) -> Namespace {
        todo!()
    }
}
