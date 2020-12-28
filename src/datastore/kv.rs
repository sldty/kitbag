use std::{
    fs,
    path::{Path, PathBuf},
    collections::HashMap,
};

use crate::{
    content::Content,
    handle::Address
};

/// A key is a String, which maps to a serializable value.
/// When stored on disk, this is files mapped to their serialized value.
/// the name can be split up, for example, consider the file:
/// ```plain
/// what/a/wonderful/world
/// ```
/// with the contenents `Hello!`
/// the key is `whatawonderfulworld`, and the value is `Hello!`
/// This allows splitting up hashes for more efficient access, like git and other things do:
/// ```
/// 79/3c77278bfed7d08829a4596eb0ddf7314d974d8c73bb364c6fb96e65741699
/// sha256 of: "git is nice"
/// ```
/// By default, DiskKV splits by the first two characters.
/// There is currently no way to override this default.
#[derive(Debug)]
pub struct DiskKV<T> {
    // The location of the datastore on-disk
    path: PathBuf,
    /// Recently accessed addresses for increased efficiency.
    /// Maps addresses to their serialized representation.
    cache: HashMap<String, Option<T>>,
}

impl<T> DiskKV<T> {
    fn scan(path: &Path) -> HashMap<String, Option<T>> {

    }

    pub fn new(path: &Path) -> Option<DiskKV<T>> {
        fs::create_dir_all(path).ok()?;

        let cache = DiskKV::scan(path);

        Some(DiskKV {
            path:  path.to_path_buf(),
            cache: HashMap::new(),
        })
    }


    pub fn has(&mut self, address: &Address) -> bool {
        todo!()
    }

    pub fn load(&self, address: &Address) -> Option<Vec<u8>> {
        todo!()
    }

    pub fn store(&mut self, content: &Content) -> Address {
        todo!()
    }
}
