use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{
    namespace::Namespace,
    traits::{Identity, Storable}
};

// TODO: make into trait or enum
// TODO: add keys to agent
#[derive(Clone, Serialize, Deserialize)]
pub struct Agent {
    display: String,
    identity: Identity, // Agent
    namespaces: HashMap<Identity, Namespace>,
}

#[typetag::serde]
impl Storable for Agent {
    fn identity(&self) -> Identity { self.identity.clone() }
    // TODO: verify cryptographic keys
    fn context(&self) -> Option<Box<dyn Storable>> { None }

    fn find(&self, identity: &Identity) -> Option<Box<dyn Storable>> {
        Some(Box::new(self.namespaces.get(identity)?))
    }
}
