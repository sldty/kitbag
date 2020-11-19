use serde::{Serialize, Deserialize};
use crate::traits::{Storable, Identity};

#[derive(Serialize, Deserialize)]
pub struct Page {
    // namespace: Namespace,
    identity: Identity,
    parent:   Identity,
    title:    String,
    branches: Vec<Identity>,
}

#[typetag::serde]
impl Storable for Page {
    fn identity(&self) -> Identity { self.identity.clone() }

    fn context(&self) -> Option<Box<dyn Storable>> {
        todo!()
    }
}
