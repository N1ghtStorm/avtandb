use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::rc::Rc;

pub struct KVStore {
    pub kv_hash_map: Arc<Mutex<HashMap<String, Arc<String>>>>
}

impl KVStore {
    pub fn new() -> Self {
        KVStore { kv_hash_map: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub fn add_value(&mut self, key: String, value: String) -> Result<(), ()> {
        // NOT SURE IF self....lock() - is a good idea
        let mut hash_map = self.kv_hash_map.lock().unwrap();
        hash_map.insert(key, Arc::new(value));
        Ok(())
    }

    pub fn get_value(&self, key: String) -> Result<Arc<String>, ()> {
        // NOT SURE IF self....lock() - is a good idea
        let hash_map = self.kv_hash_map.lock().unwrap();
        let val = hash_map.get(&key);

        return match val {
            Some(inner_val) => Ok(inner_val.clone()),
            None => Err(())
        };
    }
}