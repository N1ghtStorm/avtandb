use std::sync::Arc;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use std::collections::HashMap;

struct KVStore {
    pub kv_hash_map: Arc<Mutex<HashMap<String, String>>>
}

impl KVStore {
    fn new() -> Self {
        KVStore { kv_hash_map: Arc::new(Mutex::new(HashMap::new())) }
    }

    fn add_value(&mut self, key: String, value: String) -> Result<(), ()> {
        // NOT SURE IF self....lock() - is a good idea
        let mut hash_map = self.kv_hash_map.lock().unwrap();
        hash_map.insert(key, value);
        Ok(())
    }
}