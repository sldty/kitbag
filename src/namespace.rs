use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{
    traits::{Storable, Identity},
    agent::Agent,
    page::Page
};

#[derive(Clone, Serialize, Deserialize)]
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
