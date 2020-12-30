// Represents a peer node
pub struct Peer {
    public_key: KeyPublic,
    connection: TcpStream,
    wants:      HashSet<Address>,
    have:       HashSet<Address>,
}
