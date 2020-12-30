use crate::keys::KeyPair;

pub struct Node {
    // todo: hashmap of some sort?
    temp_key:   KeyPair,
    associates: Vec<!>,
    peers:      Vec<Peer>,
    have:       HashSet<Address>, // list of addresses this node is currently keeping track of
    want:       HashSet<Address>, // list of addresses this node is currently wanting
}

impl Node {
    pub fn new(bootstrap: Vec<!>) -> Node {
        let (initial, failed) = Node::bootstrap(bootstrap);

        Node {
            temp_key:   KeyPair::generate(),
            associates: bootstrap,
            peers:      vec![],
        }
    }
}
