use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

pub trait KVStore {
    fn add_value(&mut self, key: String, value: String) -> Result<(), ()>;
    fn get_value(&self, key: String) -> Result<Arc<String>, ()>;
    fn remove_key(&self, key: String) -> Result<(),()>;
    fn update_value(&mut self, key: String, value: String) -> Result<(),()>;
}

/// Facade over main hash map
pub struct InMemoryKVStore {
    pub kv_hash_map: Arc<Mutex<HashMap<String, Arc<String>>>>
}

impl InMemoryKVStore {
    /// ctor
    pub fn new() -> Self {
        InMemoryKVStore { kv_hash_map: Arc::new(Mutex::new(HashMap::new())) }
    }
}

impl KVStore for InMemoryKVStore {
    /// Add value to storage
    fn add_value(&mut self, key: String, value: String) -> Result<(), ()> {
        // NOT SURE IF self....lock() - is a good idea
        let mut hash_map = self.kv_hash_map.lock().unwrap();
        hash_map.insert(key, Arc::new(value));
        Ok(())
    }

    /// Get value
    fn get_value(&self, key: String) -> Result<Arc<String>, ()> {
        // NOT SURE IF self....lock() - is a good idea
        let hash_map = self.kv_hash_map.lock().unwrap();
        let val = hash_map.get(&key);

        return match val {
            Some(inner_val) => Ok(inner_val.clone()),
            None => Err(())
        };
    }

    fn remove_key(&self, key: String) -> Result<(),()> {
        todo!();
    }

    fn update_value(&mut self, key: String, value: String) -> Result<(),()> {
        todo!();
    }
}

// Stores KV on filesystem
pub struct DurableKVStore {
}

impl DurableKVStore {
    fn new() -> Self {
        DurableKVStore {}
    }
}


impl KVStore for DurableKVStore {
    /// Add value to storage
    fn add_value(&mut self, key: String, value: String) -> Result<(), ()> {
        todo!();
    }

    /// Get value
    fn get_value(&self, key: String) -> Result<Arc<String>, ()> {
        todo!();
    }

    fn remove_key(&self, key: String) -> Result<(),()> {
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

fn KVStoreFactory(kv_type: KVType) -> impl KVStore {
    return match kv_type {
        INMemory => InMemoryKVStore::new(),
        Durable => todo!()
    };
}