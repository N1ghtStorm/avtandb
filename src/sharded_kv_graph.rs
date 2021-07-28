use crate::kv_model;

pub struct KvStoreShard {
    pub sharded_hasm_map: kv_model::InMemoryKVStore
}

impl KvStoreShard {
    pub fn new() -> Self {
        KvStoreShard { sharded_hasm_map: kv_model::InMemoryKVStore::new()}
    }

    // Use it just after the shard manager had chosen this shard
    pub fn add_to_local_shard(&mut self, key: String, value: String) -> Result<(),()>{
        self.sharded_hasm_map.add_value(key, value)
    }
}