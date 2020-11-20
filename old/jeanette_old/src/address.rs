// TODO: fix the size or somethin'
/// The literal address of some data,
/// i.e. the hash of the data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address(Vec<u8>);
