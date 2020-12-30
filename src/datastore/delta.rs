pub enum Delta {
    Base {
        checksum: Address,
        data: Data
    },
    Tip  {
        checksum: Address,
        previous: Address,
        difference: Diff
    },
}
