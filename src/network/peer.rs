use std::{
    collections::HashSet,
};

// A `Peer` represents a non-local `Node`
// that we have an authenticated connection to.
pub struct Peer {
    public_key: KeyPublic,
    connection: TcpStream,
    wants:      HashSet<Address>,
    have:       HashSet<Address>,
}
