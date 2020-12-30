/// A Permission refers to how Content is synced between Forks.
pub enum Permission {
    /// Fork can not access Content
    Restricted,
    /// Fork can read Content
    Read,
    /// Fork can write (and read) Content
    Write,
}

/// `Permissions` determines whether Content is synced between forks.
/// This is veriefied through the use of a cryptographic proof log.
pub struct Permissions {
    key_perms: HashMap<Fork, Permission>,
    base_perm: Permission,
    perm_log:  Vec</* Hash of previous perm log, Fork is now Permission, Signature */>,
}
