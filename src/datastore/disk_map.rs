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
#[derive(Debug)]
pub struct DiskMap<A, B> where A: Clone, B: Storable + Clone {
    // The location of the datastore on-disk
    path: PathBuf,
    /// Recently accessed addresses for increased efficiency.
    /// Maps addresses to their serialized representation.
    cache: HashMap<A, Option<B>>,
}

impl<A, B> DiskKV<A, B> where T: Storable + Clone {
    fn scan(path: &Path) -> Option<HashMap<String, Option<T>>> {
        let mut cache = HashMap::new();

        for key_file in fs::read_dir(&path).ok()? {
            let file_path = key_file.ok()?.path();
            let name = file_path.file_name()?.to_str()?.to_string();
            cache.insert(name, None);
        }

        return Some(cache);
    }

    pub fn new(path: &Path) -> Result<DiskKV<T>, String> {
        fs::create_dir_all(path)
            .or(Err(format!("Could not create the path: {:?}", path.to_str())))?;

        Ok(DiskKV {
            path:  path.to_path_buf(),
            cache: DiskKV::<T>::scan(path)
                .ok_or(format!("Could not scan for existing keys: {:?}", path.to_str()))?,
        })
    }

    pub fn contains_key(&mut self, key: &A) -> bool {
        return self.cache.contains_key(key);
    }

    pub fn get(&self, key: &A) -> Result<T, String> {
        match self.cache.get(key) {
            Some(Some(value)) => Ok(value.to_owned()),
            Some(None) => {
                let path   = self.path.join(key); // TODO: serialize?
                let mut bytes = vec![];
                let mut file = fs::File::open(path).or(Err("Could not open key file "))?;
                file.read_to_end(&mut bytes).or(Err("Could not read key file"))?;

                let unserde: Box<T> = Storable::try_from_bytes(&bytes)
                    .ok_or("Could not deserialize value of key file")?;
                Ok(*unserde)
            },
            None => Err(String::from("Key is not on-hand")),
        }
    }

    pub fn insert(&mut self, key: &A, value: &B) -> Result<(), String> {
        let file = self.path.join(key);
        println!("{:?}", file.to_str());
        let mut file = fs::File::create(&file)
            .or(Err(format!("Could not create the key store file: {:?}", file.to_str())))?;

        let bytes = Storable::try_to_bytes(value).ok_or("Could not serialize value of key")?;
        file.write_all(&bytes).or(Err("Could not write serialized value to key file"))?;
        self.insert_temp(key, value);
        Ok(())
    }

    /// Insert an item in the DiskMap that is redundant,
    /// i.e. can be rebuilt from *different* data that is *gauranteed* to exist.
    pub fn insert_temp(&mut self, key: &A, value: &B) -> Result<(), String> {
        self.cache.insert(key.to_string(), Some(value.clone()))
    }
}
