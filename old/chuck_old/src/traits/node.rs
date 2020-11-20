use std::collections::HashMap;
use crate::traits::{
    key,
    Peer,
    Identity,
};

/// A response to be send to a peer.
/// Like Request's, Response's are contextless,
/// Meaning that they're not 'responding' to anything,
/// Merely informing a peer something
pub enum Response {
    Goodbye,
}

/// A trusted node on this machine acting on behalf of Identity's.
pub trait Node {
    fn agents(&mut self) -> HashMap<Box<dyn Identity>, Box<dyn key::Private>>;

    fn reach_out(&mut self, other: &impl Peer) -> Option<()>;
    fn   welcome(&mut self, other: &impl Peer) -> Option<()>;
    fn    handle(&mut self, other: &impl Peer) -> Option<Response>;

    fn tick(&mut self) -> Option<()>;
}
