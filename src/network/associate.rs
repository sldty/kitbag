use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use crate::keys::KeyPublic;

// A `Peer` represents a non-local `Node`
// that we do not yet have an authenticated connection to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Associate {
    pub public_key: KeyPublic,
    pub socket:    SocketAddr,
}
