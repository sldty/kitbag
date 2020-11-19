use std::{
    path::PathBuf,
    collections::HashMap,
};

use crate::traits::{
    Address,
    Blob,
    Storable,
    Identity,
};

pub struct Datastore {
    path: PathBuf,
    cached_addressed: HashMap<Address, Blob>,
    // agents: Vec<Agent>,
}
