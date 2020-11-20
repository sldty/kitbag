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

#[typetag::serde]
impl Storable for Namespace {
    fn identity(&self) -> Identity { self.identity.clone() }

    fn context(&self) -> Option<Box<dyn Storable>> {
        Some(Box::new(self.agent.clone()))
    }

    fn find(&self, identity: &Identity) -> Option<Box<dyn Storable>> {
        Some(Box::new(self.pages.get(identity)?.clone()))
    }
}
