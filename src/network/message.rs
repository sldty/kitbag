use serde::{Serialize, Deserialize};
use crate::network::Associate;
use crate::handle::Address;
use crate::keys::KeyPublic;

// NOTE: nodes are responseless, which means after a secure connection has been
// established, messages are sent without expecting a response, necessarily.
// this makes things easier for now, but a request-response model is
// definitely something to look into in the future.

// TODO: should all messages be encrypted?

/// Represents a `Message`, that should be encrypted, sent between `Node`s.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    // All messages are encrypted except for g'day
    // Two layers of encryption:
    // temp_key by default
    // Data's Vec<u8> is encrypted with a specific key

    // Ask and send out a list of the peers a node is connected to
    Boostrap,
    Peers(Vec<Associate>),

    // Ask and send out who has an address
    Who(Address),
    Has(Address, Vec<Associate>),

    // Ask and send out some data
    Want(KeyPublic, Address),
    Data(KeyPublic, Address, Vec<u8>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Payload {
    KeyGday(KeyPublic),
    Encrypted {
        nonce:     [u8; 12],
        encrypted: Vec<u8>,
    }
}
