use rand_core::{RngCore, OsRng};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};

use crate::keys::{KeyPair, KeyPublic};

// NOTE: do no implement serde for this
// Represents a symmetrical secret key,
// used by ChaCha20Poly1305 - generate one through
// the use of the associated method `shared_secret` on `KeyPair`
pub struct KeySecret([u8; 32]);

impl KeySecret {
    pub fn new(bytes: &[u8; 32]) -> KeySecret {
        KeySecret(bytes.to_owned())
    }

    fn bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    // TODO: sign message
    /// Using a secret key pair and the recipient's public key,
    /// this function encrypts a message for them,
    /// returning the nonce and the encrypted message.
    /// Uses ChaCha20Poly1305 - HKDF + ECC are used to generate the shared secret.
    pub fn encrypt_message(
        key_pair:   &KeyPair,
        key_public: &KeyPublic,
        message:    Vec<u8>,
    ) -> Option<([u8; 12], Vec<u8>)> {
        let secret = key_pair.shared_secret(key_public)?;
        let key = Key::from_slice(&secret.bytes());

        let mut proto_nonce = [0u8; 12];
        OsRng.fill_bytes(&mut proto_nonce);
        let nonce = Nonce::from_slice(&proto_nonce);

        let cipher = ChaCha20Poly1305::new(key);
        let encrypted = cipher.encrypt(&nonce, message.as_ref()).ok()?;
        return Some((proto_nonce, encrypted));
    }

    /// Using a secret key pair, the recipient's public key, and the message nonce,
    /// this function decrypts a message from them,
    /// returning the decrypted message.
    /// To be used with `encrypt_message`.
    pub fn decrypt_message(
        key_pair:    &KeyPair,
        key_public:  &KeyPublic,
        proto_nonce: [u8; 12],
        encrypted:   Vec<u8>,
    ) -> Option<Vec<u8>> {
        let secret = key_pair.shared_secret(key_public)?;
        let key = Key::from_slice(&secret.bytes());
        let nonce = Nonce::from_slice(&proto_nonce);

        let cipher = ChaCha20Poly1305::new(key);
        let message = cipher.decrypt(&nonce, encrypted.as_ref()).ok()?;
        return Some(message);
    }

}
