use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;

pub trait KVStore {
    fn add_value(&mut self, key: String, value: String) -> Result<(), ()>;
    fn get_value(&self, key: String) -> Result<Arc<String>, ()>;
    fn remove_key(&mut self, key: String) -> Result<(),()>;
    fn update_value(&mut self, key: String, value: String) -> Result<(),()>;
}

/// Facade over main hash map
pub struct InMemoryKVStore {
    pub kv_hash_map: Arc<RwLock<HashMap<String, Arc<String>>>>
}

pub struct ShardedKvStore {
    pub sharded_hasm_map: InMemoryKVStore
}

impl InMemoryKVStore {
    /// ctor
    pub fn new() -> Self {
        InMemoryKVStore { kv_hash_map: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub fn add_value(&mut self, key: String, value: String) -> Result<(), ()> {
        // NOT SURE IF self....lock() - is a good idea
        let mut hash_map = self.kv_hash_map.write().unwrap();
        hash_map.insert(key, Arc::new(value));
        Ok(())
    }

    /// Get value
    pub fn get_value(&self, key: String) -> Result<Arc<String>, ()> {
        // NOT SURE IF self....lock() - is a good idea
        let hash_map = self.kv_hash_map.read().unwrap();
        let val = hash_map.get(&key);

        return match val {
            Some(inner_val) => Ok(inner_val.clone()),
            None => Err(())
        };
    }

    /// Removes Key-Value Pair from KV collection
    pub fn remove_key(&mut self, key: String) -> Result<(),()> {
        // NOT SURE IF self....lock() - is a good idea
        let mut hash_map = self.kv_hash_map.write().unwrap();
        match hash_map.remove(&key) {
            Some(_) => Ok(()),
            None => Err(())
        }
    }

    /// Updates the value by key
    pub fn update_value(&mut self, key: String, value: String) -> Result<(),()> {
        // NOT SURE IF self....lock() - is a good idea
        let mut hash_map = self.kv_hash_map.write().unwrap();
        match hash_map.get(&key){
            None => Err(()),
            Some(_) => {
                hash_map.insert(key, Arc::new(value));
                Ok(())
            }
        }
    }

    /// Get all Keys Collection
    pub fn get_all_keys(&self) -> Result<Vec<String>, ()> {
        // NOT SURE IF self....lock() - is a good idea
        let mut hash_map = self.kv_hash_map.read().unwrap();

        // TODO: think about .clone() ?????
        let vals: Vec<String> = hash_map.iter().map(|(x, _)| x.clone()).collect();
        Ok(vals)
    }

    /// Renames Key
    pub fn rename_key(&mut self, key: String, new_key: String) -> Result<(), ()> {
        // NOT SURE IF self....lock() - is a good idea
        let mut hash_map = self.kv_hash_map.write().unwrap();
        match hash_map.remove(&key) {
            None => Err(()),
            Some(stored_val) => {
                hash_map.remove(&key);
                hash_map.insert(new_key, stored_val);
                Ok(())
            }
        }
    }
}

// Stores KV on filesystem
pub struct DurableKVStore {
}

impl DurableKVStore {
    fn new() -> Self {
        todo!();
    }
}

impl KVStore for DurableKVStore {
    /// Add value to disk storage
    fn add_value(&mut self, key: String, value: String) -> Result<(), ()> {
        todo!();
    }

    /// Get value
    fn get_value(&self, key: String) -> Result<Arc<String>, ()> {
        todo!();
    }

    fn remove_key(&mut self, key: String) -> Result<(),()> {
        todo!();
    }

    fn update_value(&mut self, key: String, value: String) -> Result<(),()> {
        todo!();
    }
}

/// To choose which KV type to use
pub enum KVType {
    INMemory,
    Durable
}