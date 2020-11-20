use crate::traits::node::Response;

pub enum Request {
    Goodbye,
}

/// An untrusted but communicable peer, purpotedly acting on behalf of some account
pub trait Peer {
    fn    send(&mut self, request: Request) -> Option<()>;
    fn recieve(&mut self) -> Option<Response>;
}
