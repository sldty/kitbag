use serde::{Serialize, Deserialize};

use std::collections::HashMap;

use crate::{
    handle::Address,
    content::Content,
    datastore::{Storable, Delta},
};

/// Represents a single chain of versions.
#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    /// The Address of the latest Delta.
    pub head: Address,
    /// Maps Content addresses to the Delta that is used to make that address.
    deltas: HashMap<Address, Delta>,
}

impl Clone for History {
    fn clone(&self) -> History {
        // please don't kill me
        // TODO: actually clone it better
        *Storable::try_from_bytes(&Storable::try_to_bytes(self).unwrap()).unwrap()
    }
}

impl History {
    /// Create a new history.
    pub fn new(initial: Content) -> Option<History> {
        let address = Address::new(&Storable::try_to_bytes(&initial)?);
        let delta = Delta::base(initial)?;

        let mut deltas = HashMap::new();
        deltas.insert(address.clone(), delta);

        return Some(History { head: address, deltas });
    }

    /// Commit a delta onto the head history.
    /// Returns None if the delta can not be applied,
    /// Panics if it is passed a base delta, which should be unreachable.
    pub fn commit(&mut self, previous: &Content, next: &Content) -> Option<()> {
        let delta = Delta::make(previous, next)?;

        let address = if let Delta::Tip { previous, checksum, .. } = &delta {
            if previous != &self.head { return None; }
            checksum.clone()
        } else { unreachable!() };

        self.deltas.insert(address.clone(), delta);
        self.head = address;
        return Some(());
    }

    pub fn delta(&self, address: &Address) -> Option<&Delta> {
        let delta = self.deltas.get(address)?;
        return Some(delta);
    }
}
