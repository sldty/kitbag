use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::Account;

#[repr(u8)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Permission {
    /// Can not even see that the document exists.
    None = 0,
    /// Can view the document, but can not edit it.
    Viewer,
    /// Can edit the document, but can not move or delete it.
    Editor,
    /// Can move or delete the document.
    Owner,
}

/// Represents the permissions of a specific document
#[derive(Clone, Serialize, Deserialize)]
pub struct Permissions {
    /// Is the permanent owner, unless ownership is transfered.
    account: Account,
    /// The base permission of all accounts. Can not be less than this.
    base: Permission,
    /// Permissions for specific accounts.
    permissions: HashMap<Account, Permissions>,
}

impl Permissions {
    pub fn new(account: Account, base: Permission) -> Permissions {
        Permissions {
            account,
            base,
            permissions: HashMap::new(),
        }
    }
}
