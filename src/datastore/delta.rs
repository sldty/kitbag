use crate::{
    handle::Address,
    data::Data,
    diff::Diff,
};

/// Represents the change between two versions.
/// Note that a checksum is stored, so this `Delta`'s `Diff` can *not* be applied
/// willy-nilly irrespective to any `Data`.
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
