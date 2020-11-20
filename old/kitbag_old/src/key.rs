use serde::{Serialize, Deserialize};

pub fn hash(data: &[u8]) -> Vec<u8> {
    use sha3::{Digest, Sha3_256};

    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let result: Vec<u8> = hasher.finalize().as_slice().to_vec();
    return result;
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Digest([u8; 32]);

/// Represents a public key and that key's digest.
#[derive(Clone, Serialize, Deserialize)]
pub struct PublicKey {
    digest: Digest,
    public: [u8; 32],
}

impl PublicKey {
    pub fn new(public: [u8; 32], digest: Digest) -> Option<PublicKey> {
        let potential_key = PublicKey { digest: digest.clone(), public };
        // check that the digest is the hash of the public key
        match potential_key.verify(digest) {
            true  => Some(potential_key),
            false => None,
        }
    }

    fn verify(&self, digest: Digest) -> bool {
        Digest(hash(&self.public)) == digest && digest == self.digest
    }
}

/// Represents a private key, separate from a public key.
pub struct PrivateKey([u8; 32]);

impl PrivateKey {
    pub fn generate() -> PrivateKey {
        todo!()
    }

    pub fn derive_public(&self) -> PublicKey {
        todo!()
    }

    pub fn verify(&self, public_key: PublicKey) -> bool {
        // generate public_key from self, compare
        todo!()
    }
}
