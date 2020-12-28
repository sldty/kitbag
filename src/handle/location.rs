use serde::{Serialize, Deserialize};
use crate::handle::Identity;

/// A Location is a chain of identities leading to an identity
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location(Vec<Identity>);

impl Location {
    pub fn root() -> Location { Location(vec![]) }

    pub fn cd(&self, identity: &Identity) -> Location {
        let mut chain = self.0.clone();
        chain.push(identity.clone());
        return Location(chain);
    }

    pub fn to_string(&self) -> String {
        self.0.iter()
            .map(|i| i.tag().hex())
            .collect::<Vec<String>>()
            .join("/")
    }
}
