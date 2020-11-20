use crate::{
    Address,
    Content,
    Diff,
    Agent,
};

#[derive(Debug, Clone)]
pub enum Version {
    Base(Content),
    Head {
        local_for:     Agent,
        /// Optional because we might not have
        /// the content on-hand for this particular version.
        diff:          Diff,
        previous:      Address,
        prev_resolved: Option<Box<Version>>,
        address:       Address,
        addr_resolved: Option<Content>,
    }
}

impl Version {
    // pub fn commit(&self, content: Content) -> Option<Version> {
    //
    // }
}
