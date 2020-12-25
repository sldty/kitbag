use crate::{
    handle::Address,
    diff::Diff,
    content::Content,
};

// TODO: make deltas content so that they can be resolved!

#[derive(Debug)]
pub enum Delta {
    /// The initial version, to which all changes are applied.
    Base {
        /// The base unit of content itself upon which all deltas are applied.
        base: Content,
        /// A hash of the base unit of content
        checksum: Address,
    },
    /// A tip applied to either a base (initial version) or another tip
    /// to create a new content
    Tip {
        /// The address of the previous version's content.
        previous: Address,
        /// A diff that can be applied to the previous version to get the next version.
        difference: Diff,
        /// A hash of the content after the diff is applied
        checksum: Address,
    }
}

/// Generated deltas must be stored in the datastore!
impl Delta {
    pub fn base(initial: Content) -> Option<Delta> {
        let checksum = Address::stamp(&initial)?.0;
        Some(Delta::Base { base: initial, checksum })
    }

    // calculate the diffs and addresses
    pub fn make(previous: &Content, next: &Content) -> Option<Delta> {
        let prev_addr = Address::stamp(previous)?.0;
        let next_addr = Address::stamp(next)?.0;
        let diff = Diff::make(previous, next)?;

        Some(Delta::Tip {
            previous:   prev_addr,
            difference: diff,
            checksum:   next_addr,
        })
    }
}
