use std::{
    net::TcpListener,
    collections::HashSet,
};
use crate::{
    keys::KeyPair,
    handle::Address,
    network::{Peer, Associate},
};

/// A `Node` is a local participant of the network we have control over.
/// It acts on behalf of a `Datastore`.
pub struct Node {
    // todo: hashmap of some sort?
    temp_key:   KeyPair,
    listener:   TcpListener,
    associates: Vec<Associate>,
    peers:      Vec<Peer>,
    have:       HashSet<Address>, // list of addresses this node is currently keeping track of
    want:       HashSet<Address>, // list of addresses this node is currently wanting
}

impl Node {
    pub fn new(bootstrap: Vec<Associate>) -> Node {
        let (initial, failed) = Node::bootstrap(bootstrap);

        todo!()
    }
}
