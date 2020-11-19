use serde::{Serialize, Deserialize};
use crate::{namespace::Namespace, traits::{Storable, Identity}};

#[derive(Clone, Serialize, Deserialize)]
pub struct Page {
    namespace: Namespace,
    identity:  Identity, // Page
    parent:    Identity, // Page
    title:     String,
    // TODO: move branches to datastore?
    // Have datastore be the base identity?
    // branches:  Vec<Identity<Branch>>,
}

#[typetag::serde]
impl Storable for Page {
    fn identity(&self) -> Identity { self.identity.clone() }

    fn context(&self) -> Option<Box<dyn Storable>> {
        todo!()
    }
}
