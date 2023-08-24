use std::collections::HashMap;

/// KvStore struct
// https://doc.rust-lang.org/std/default/trait.Default.html
#[derive(Default)]
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    /// instantiate a KvStore instance
    pub fn new() -> KvStore {
        KvStore {
            data: HashMap::new(),
        }
    }

    /// Set the value of a string key to a string
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Get the string value of a string key
    /// If the key does not exist, return None
    pub fn get(&self, key: String) -> Option<String> {
        if let Some(value) = self.data.get(&key) {
            return Some(value.clone());
        }
        None
    }

    /// Remove a given key
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}
