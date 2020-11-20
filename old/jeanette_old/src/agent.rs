use crate::Namespace;

// TODO: make enum?
// TODO: Agents are the same if their digests match,
// not when their names do. Implement Eq.
/// Represents one or more people (or machines)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Agent {
    pub name: String,
    // TODO: ideally agent's identity is represented
    // by the digest of their public key.
    // TODO: cryptographic keys and so on?
}

impl Agent {
    pub fn new(name: &str) -> Agent {
        Agent { name: name.to_string() }
    }
}
