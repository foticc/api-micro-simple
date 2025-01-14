use std::collections::HashMap;
use std::sync::RwLock;
use once_cell::sync::Lazy;

static CACHE:Lazy<Cache> = Lazy::new(||
    Cache::new()
);

pub struct Cache {
    data: RwLock<HashMap<String, String>>,
}

impl Cache {

    pub fn set_cache(key:String, value:String) {
        CACHE.set(key,value)
    }

    pub fn get_cache(key:String)->Option<String> {
        CACHE.get(key.as_str())
    }

    pub fn remove_cache(key:String)->Option<String> {
        CACHE.remove(key.as_str())
    }

    fn new() -> Self {
        Cache {
            data: RwLock::new(HashMap::new()),
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        let data = self.data.read().unwrap();
        data.get(key).cloned() // 返回值的克隆
    }

    fn set(&self, key: String, value: String) {
        let mut data = self.data.write().unwrap();
        data.insert(key, value);
    }

    fn remove(&self, key: &str) -> Option<String> {
        let mut data = self.data.write().unwrap();
        data.remove(key)
    }
}