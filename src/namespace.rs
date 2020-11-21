use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use crate::{
    agent::Agent,
    page::Page,
    handle::{Location, Identity}, data::Data
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Namespace {
    /// The owner of the namespace
    agent:    Location,
    pub identity: Identity, // Namespace
    pub title:    String,
    roots:    Vec<Identity>, // Page
    pages:    HashSet<Identity>, // Page
}

impl Namespace {
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

    pub fn register_page(&mut self, page: &Page) {
        self.pages.insert(page.identity.clone());
    }

    pub fn location(&self) -> Location { self.agent.find(&self.identity) }
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
