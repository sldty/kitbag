use serde::{Serialize, Deserialize};
use crate::{traits::{Storable, Identity}, page::Page, agent::Agent};

#[derive(Clone, Serialize, Deserialize)]
pub struct Namespace {
    /// The owner of the namespace
    agent:    Agent,
    identity: Identity, // Namespace
    title:    String,
    root:     Identity, // Page
}

#[typetag::serde]
impl Storable for Namespace {
    fn identity(&self) -> Identity { self.identity.clone() }

    fn context(&self) -> Option<Box<dyn Storable>> {
        Some(Box::new(self.agent.clone()))
    }
}
