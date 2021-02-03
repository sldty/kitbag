use std::{
    io::{Read, Write},
    net::{TcpStream, TcpListener, SocketAddr, Shutdown},
    collections::HashSet,
};
use crate::{
    keys::KeyPair,
    handle::Address,
    network::{Peer, Associate, Message, Payload},
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
    pub fn new(socket: SocketAddr, bootstrap: Vec<Associate>) -> Option<Node> {
        Node::new_with_key(KeyPair::generate(), socket, bootstrap)
    }

    pub fn new_with_key(
        key_pair: KeyPair,
        socket: SocketAddr,
        bootstrap: Vec<Associate>
    ) -> Option<Node> {
        let mut node = Node {
            temp_key: key_pair,
            listener: TcpListener::bind(socket).ok()?,
            associates: vec![],
            peers: vec![],
            have: HashSet::new(),
            want: HashSet::new(),
        };
        node.bootstrap(bootstrap);
        return Some(node);
    }

    /// Goes through each associate,
    /// and tries to establish a connection.
    pub fn bootstrap(&mut self, bootstrap: Vec<Associate>) {
        for associate in bootstrap.into_iter() {
            if let Some(peer) = self.connect(&associate) {
                self.peers.push(peer);
            } else {
                self.associates.push(associate);
            }
        }
    }

    pub fn connect(&mut self, associate: &Associate) -> Option<Peer> {
        let mut stream = TcpStream::connect(associate.socket).ok()?;
        stream.set_nodelay(true).ok()?;

        // Handshake

        // Send public key
        let key_public_send = Payload::Gday(self.temp_key.public_bytes());
        stream.write(&rmp_serde::to_vec(&key_public_send).ok()?).ok()?;

        // get public key and check that it matches
        let key_public_recv = match rmp_serde::from_read(&stream).ok()? {
            Payload::Gday(key) if key == associate.public_key => key,
            _ => { return None; },
        };

        // construct peer
        let peer = Peer {
            key_public: key_public_recv,
            connection: stream,
            wants: HashSet::new(),
            have:  HashSet::new(),
        };

        return Some(peer);
    }
}
