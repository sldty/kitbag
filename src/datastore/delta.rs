use crate::{
    handle::Address,
    data::Data,
    diff::Diff,
};

pub enum Delta {
    Base {
        checksum: Address,
        data: Data
    },
    Tip  {
        checksum: Address,
        previous: Address,
        difference: Diff,
    },
}
