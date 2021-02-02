use std::{
    io::{Read, Write},
    net::{TcpStream, TcpListener, SocketAddr},
    collections::HashSet,
};
use crate::{
    keys::KeyPair,
    handle::Address,
    network::{Peer, Associate, Message},
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
        let key_public_send = Message::Gday(self.temp_key.public_bytes());
        Node::send_message(&mut stream, &key_public_send);

        // get public key and check that it matches
        let key_public_recv = match Node::next_message(&stream)? {
            Message::Gday(key) if key == associate.public_key => key,
            _ => return None,
        };

        // generate shared secret
        let shared_secret = self.temp_key.shared_secret(&key_public_recv)?;

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
