use serde::{Serialize, Deserialize};

use crate::{
    handle::Address,
    diff::Diff,
    datastore::Storable,
};

// TODO: make deltas content so that they can be resolved!

/// A Delta is either a Base or a Tip.
/// A Base is the initial version, to which all changes are applied.
/// A Tip is applied to either a Base (initial version) or another Tip
/// to create a new content
#[derive(Debug, Serialize, Deserialize)]
pub enum Delta {
    Base {
        base:     Content,
        checksum: Address,
    },
    Tip {
        previous:   Address,
        difference: Diff,
        checksum:   Address,
    }
}

/// Generated deltas must be stored in the datastore!
impl Delta {
    pub fn base(initial: Content) -> Option<Delta> {
        let checksum = Address::new(&Storable::try_to_bytes(&initial)?);
        Some(Delta::Base { base: initial, checksum })
    }

    // calculate the diffs and addresses
    pub fn make(previous: &Content, next: &Content) -> Option<Delta> {
        let prev_addr = Address::new(&Storable::try_to_bytes(previous)?);
        let next_addr = Address::new(&Storable::try_to_bytes(next)?);
        let diff = Diff::make(previous, next)?;

        Some(Delta::Tip {
            previous:   prev_addr,
            difference: diff,
            checksum:   next_addr,
        })
    }
}
