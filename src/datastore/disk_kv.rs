use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    collections::HashMap,
};

use crate::datastore::Storable;

/// TODO append three characters to end of every key before writing?
/// A stupid append-only HashMap that writes to disk.
/// A key is a (utf-8) String, which maps to a ~~serializable~~ `Storable` value.
/// When stored on disk, this is files mapped to their serialized value.
/// DiskKV allows splitting up hashes for more efficient access, like git and other things do:
/// ```
/// 79/3c77278bfed7d08829a4596eb0ddf7314d974d8c73bb364c6fb96e65741699
/// sha256 of: "git is nice"
/// ```
/// By default, DiskKV splits by the first two characters.
/// There is currently no way to override this default.
/// This means all keys must be at least three characters long.
#[derive(Debug)]
pub struct DiskKV<T> where T: Storable + Clone {
    // The location of the datastore on-disk
    path: PathBuf,
    /// Recently accessed addresses for increased efficiency.
    /// Maps addresses to their serialized representation.
    cache: HashMap<String, Option<T>>,
}

impl<T> DiskKV<T> where T: Storable + Clone {
    fn scan(path: &Path) -> Result<HashMap<String, Option<T>>, String> {
        let mut cache = HashMap::new();

        for sub_folder in fs::read_dir(&path)
            .or(Err(format!("Could not read the folders in the directory {:?}", path.to_str())))?
        {
            let path = &sub_folder
                .or(Err("Item in directory can't be used"))?
                .path();
            let name_start = path.file_name()
                .ok_or("Could not determine folder name")?
                .to_str()
                .ok_or("Could not convert file name to string")?
                .to_string();

            for key_file in fs::read_dir(&path)
                .or(Err(format!("Could not read the files in the directory {:?}", path.to_str())))?
            {
                let name_end = &key_file
                    .or(Err("Item in directory can't be used"))?
                    .path().file_name()
                    .ok_or("Could not determine the file name")?
                    .to_str()?
                    .ok_or("Could not convert file name to string")?
                    .to_string();
                cache.insert(name_start.clone() + &name_end, None);
            }
        }

        return Some(cache);
    }

    pub fn new(path: &Path) -> Result<DiskKV<T>, String> {
        fs::create_dir_all(path)
            .or(Err(format!("Could not create the path: {:?}", path.to_str())))?;

        Ok(DiskKV {
            path:  path.to_path_buf(),
            cache: DiskKV::<T>::scan(path)?,
        })
    }

    pub fn has(&mut self, key: &str) -> bool {
        return self.cache.contains_key(key);
    }

    pub fn load(&self, key: &str) -> Option<T> {
        match self.cache.get(key) {
            Some(Some(value)) => Some(value.to_owned()),
            Some(None) => {
                let folder = &key[..2];
                let name   = &key[2..];
                let path   = self.path.join(folder).join(name);

                let mut bytes = vec![];
                let mut file = fs::File::open(path).ok()?;
                file.read_to_end(&mut bytes).ok()?;

                let unserde: Box<T> = Storable::try_from_bytes(&bytes)?;
                Some(*unserde)
            },
            None => None,
        }
    }

    pub fn store(&mut self, key: &str, value: &T) -> Option<()> {
        let folder = &key[..2];
        let name   = &key[2..];
        let dir    = self.path.join(folder);
        let file   = dir.join(name);

        fs::create_dir(folder).ok()?;
        let mut file = fs::File::create(file).ok()?;

        let bytes = Storable::try_to_bytes(value)?;
        file.write_all(&bytes).ok()?;
        Some(())
    }
}
