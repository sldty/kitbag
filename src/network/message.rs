// nodes are responseless, which means after a secure connection has been
// established, messages are sent without expecting a response, necessarily.
// this makes things easier for now, but a request-response model is
// definitely something to look into in the future.
pub enum Message {
    // authenticate connection
    // Gday(Vec<>),

    // Ask and send out a list of the peers a node is connected to
    Boostrap,
    Peers(Vec<Peer>),

    // Ask and send out who has an address
    Who(Address),
    Has(Address, Vec<Peer>),

    // Ask and send out some data
    Want(KeyPublic, Address),
    Data(KeyPublic, Address, Vec<u8>),

    // Terminate connection
    Goodbye,
}
