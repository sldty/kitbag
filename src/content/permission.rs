pub enum Permission {
    Restricted,
    Read,
    Write,
}

pub struct Permissions {
    key_perms: HashMap<Fork, Permission>,
    base_perm: Permission,
    perm_log:  Vec</* Hash of previous perm log, Fork is now Permission, Signature */>,
}
