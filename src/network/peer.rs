use std::{
    io::{Read, Write},
    net::TcpStream,
    collections::HashSet,
};
use crate::keys::{KeyPublic, KeyPair, KeySecret};
use crate::handle::Address;
use crate::network::{Payload, Message, Associate};

/// A `Peer` represents a non-local `Node`
/// that we have an authenticated connection to.
pub struct Peer {
    pub key_public:    KeyPublic,
    pub connection:    TcpStream,
    pub wants:         HashSet<Address>,
    pub have:          HashSet<Address>,
}

// TODO: get rid of payload and just use Message?

impl Peer {
    /// Given a Node's temp_key, decrypts the next encrypted message from a peer
    pub fn next_message(&mut self, key_pair: KeyPair) -> Option<Message> {
        let payload = rmp_serde::from_read(&self.connection).ok()?;

        let (nonce, encrypted) = match payload {
            Payload::Gday(_) => return None,
            Payload::Encrypted { nonce, encrypted } => (nonce, encrypted),
        };

        let serialized = KeySecret::decrypt_message(
            &key_pair,
            &self.key_public,
            nonce,
            encrypted,
        )?;

        let message = rmp_serde::from_read_ref(&serialized).ok()?;
        return Some(message);
    }

    /// Given a Node's temp_ley, encrypts a message to a peer
    pub fn send_message(&mut self, key_pair: KeyPair, message: &Message) -> Option<()> {
        let serialized = rmp_serde::to_vec(message).ok()?;

        let (nonce, encrypted) = KeySecret::encrypt_message(
            &key_pair,
            &self.key_public,
            serialized,
        )?;

        let payload = Payload::Encrypted { nonce, encrypted };

        self.connection.write(&rmp_serde::to_vec(&payload).ok()?).ok()?;
        Some(())
    }
}
