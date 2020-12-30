// A `Peer` represents a non-local `Node`
// that we do not yet have an authenticated connection to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Associate {
    public_key: KeyPublic,
    address:    TcpAddress,
}
