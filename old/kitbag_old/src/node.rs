use rand::random;
use std::{
    io::prelude::*,
    collections::HashSet,
    net::{
        TcpListener,
        TcpStream,
        SocketAddr,
    }
};
use serde::{Serialize, Deserialize, Deserializer};
use crate::{
    Account,
    Address,
    PublicKey,
    PrivateKey,
};

// TODO: finalize messages

const LISTENER: &str = "127.0.0.1:42069";

#[derive(Serialize, Deserialize)]
pub enum Request {
    Connect {
        public_key: PublicKey,
        favor: Option<i64>,
    },
    WhoHas(Box<Address>),
    Want(Box<Address>),
    Goodbye,
}

// TODO: Much variants for sending large blocks of data and peers?
#[derive(Serialize, Deserialize)]
pub enum Response {
    Handshake {
        public_key: PublicKey,
        favor: Option<i64>
    },
    Ignored,
    Peer((Vec<u8>, SocketAddr)),
    Have(Box<Address>),
    Data(Address, Vec<u8>),
    Goodbye,
}

pub struct Peer {
    network_address: SocketAddr,
    connection: TcpStream,
    has: HashSet<Address>,
    wants: HashSet<Address>,
    favor: i64,
    public_key: PublicKey,
}

impl Peer {
    pub fn did_it(&mut self) { self.favor += 1; }
    pub fn failed(&mut self) { self.favor -= 1; }

    pub fn send(&mut self, request: Request) -> Option<()> {
        let (_, serialized) = Address::hash(&request);
        self.connection.write(&serialized).ok()?;
        Some(())
    }


    pub fn recieve(&mut self) -> Option<Response> {
        let mut it = Deserializer::from_reader(self.connection);
        todo!()
    }

    // read ipfs paper on bitswap, implement similar protocol.

    /// Yay! a sigmoid
    pub fn friendliness(&self) -> u8 {
        // between 0 and 1:
        let squashed = 1.0 / (1.0 + (2.72 as f64).powf(-self.favor as f64));
        (squashed * 255.0) as u8
    }

    pub fn sample_friend(&self) -> bool {
        let sample: u8 = random();
        return sample >= self.friendliness();
    }
}

// TODO: some sort of priority queue
// to keep track of which peers have incoming messages
// which are connected,
// etc.

/// Represents a signle node in the solidarity network.
/// Note that accounts are federated across nodes.
pub struct Node {
    accounts:    Vec<Account>,
    listener:    TcpListener,
    potentials:  Vec<SocketAddr>,
    peers:       Vec<Peer>,
    has:         HashSet<Address>,
    wants:       HashSet<Address>,
    public_key:  PublicKey,
    private_key: PrivateKey,
}

impl Node {
    pub fn initialize(
        address: SocketAddr,
        private_key: PrivateKey,
        bootstrap: Vec<SocketAddr>,
    ) -> Option<Node> {
        let listener = TcpListener::bind(LISTENER).ok()?;
        let public_key = private_key.derive_public();

        Some(
            Node {
                accounts: vec![],
                listener: listener,
                potentials: bootstrap,
                // TODO: bootstrap?
                peers: vec![],
                // TODO: read datastore
                has:   HashSet::new(),
                wants: HashSet::new(),
                public_key,
                private_key,
            }
        )
    }

    /// Try to initiate a connection with a peer
    pub fn greet(network_address: SocketAddr) -> Option<()> {
        // establish connection
        let mut connection = TcpStream::connect(&network_address).ok()?;
        connection.set_nodelay(true).ok()?;
        connection.set_nonblocking(true).ok()?;

        todo!()
    }

    /// Add a peer when
    pub fn handshake(network_address: SocketAddr) -> Option<()> {
        todo!()
    }

    pub fn accept_incoming() -> Option<()> {
        todo!()
    }

    pub fn handle(&mut self, peer: Peer) {
        todo!()
    }

    pub fn request(&mut self, peer: Peer) {
        todo!()
    }
}
