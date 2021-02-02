use serde::{Serialize, Deserialize};
use crate::{handle::Fork, data::Data};
use super::permission::Permissions;

/// `Content` refers to some `Data` at a specific version
/// within the context of a `Fork`.
/// This can be though of as the metadata that:
/// - Keeps track of the versions of some `Data`
/// - Keeps track of the permissions of related Forks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    main_fork: Fork,
    sync:      Permissions,
    data:      Data,
}
