use std::collections::HashMap;
use std::sync::RwLock;

use std::sync::atomic::{AtomicU64, Ordering};

static CACHE_SEQ: AtomicU64 = AtomicU64::new(1);

struct CacheEntry<T: Clone> {
    data: Vec<T>,
    id: u64,
}

pub struct GenericCache<T: Clone> {
    data_cache: RwLock<HashMap<String, CacheEntry<T>>>,
    limit_mb: usize,
}

impl<T: Clone> GenericCache<T> {
    pub fn new(limit_mb: usize) -> Self {
        Self {
            data_cache: RwLock::new(HashMap::new()),
            limit_mb,
        }
    }

    pub fn read_cache(&self, cache_key: &str) -> Option<Vec<T>> {
        self.data_cache
            .read()
            .unwrap()
            .get(cache_key)
            .map(|x| x.data.clone())
    }

    pub fn write_cache(&self, cache_key: &str, data: Vec<T>) {
        self.data_cache.write().unwrap().insert(
            cache_key.to_string(),
            CacheEntry {
                id: CACHE_SEQ.fetch_add(1, Ordering::Relaxed),
                data,
            },
        );
        self.cleanup();
    }

    pub fn purge_cache(&self, cache_key: &str) {
        // println!("remove {cache_key}");
        self.data_cache.write().unwrap().remove(cache_key);
    }

    pub fn cleanup(&self) {
        while self.count_size() > (self.limit_mb * 1024 * 1024) as u64 {
            self.remove_oldest_item();
        }
    }

    pub fn remove_oldest_item(&self) {
        let cache = self.data_cache.read().unwrap();
        let lowest = cache.iter().fold((u64::MAX, ""), |p, (key, entry)| {
            if entry.id < p.0 { (entry.id, key) } else { p }
        });
        let key = lowest.1.to_string();
        println!("remove as oldest {key}");
        drop(cache);
        self.data_cache.write().unwrap().remove(&key);
    }

    pub fn count_size(&self) -> u64 {
        let cache = self.data_cache.read().unwrap();
        cache.iter().fold(0, |p, (_, entry)| {
            entry.data.len() as u64 * size_of::<T>() as u64 + p
        })
    }
}
