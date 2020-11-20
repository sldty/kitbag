use serde::{Serialize, Deserialize};
use crate::traits::{
    key,
    Identity,
    Content,
    Storable,
};

#[derive(Serialize, Deserialize)]
pub struct Account where {
    name: String,
    public_key: Box<dyn key::Public>,
}

#[typetag::serde] impl Content for Account {}
#[typetag::serde] impl Storable for Account {}

impl Identity for Account {
    fn name(&self) -> String { self.name.to_string() }
    fn public(&self) -> Box<dyn key::Public> where Self: Sized {
        self.public_key.clone()
    }
}

impl Account {
    pub fn create(name: String, public_key: Box<dyn key::Public>) -> Account {
        Account {
            name,
            public_key,
        }
    }
}
