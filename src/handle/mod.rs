pub mod address;
pub mod identity;
pub mod location;

pub use address::Address;
pub use identity::Identity;
pub use location::{Fork, Location};

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}
