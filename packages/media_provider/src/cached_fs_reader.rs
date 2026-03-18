use std::collections::HashMap;
use std::path::Path;
use std::sync::RwLock;
use std::{fs, slice};

use std::sync::atomic::{AtomicU64, Ordering};

static CACHE_SEQ: AtomicU64 = AtomicU64::new(1);

static CACHE_SIZE_LIMIT: u64 = 2 * 1024 * 1024 * 1024; // 2 gb

struct CacheEntry {
    data: Vec<u8>,
    id: u64,
}

pub struct CachedFSReader {
    data_cache: RwLock<HashMap<String, CacheEntry>>,
}

impl Default for CachedFSReader {
    fn default() -> Self {
        Self::new()
    }
}

impl CachedFSReader {
    pub fn new() -> CachedFSReader {
        Self {
            data_cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn read_media_file(&self, media_path: &Path) -> Vec<u8> {
        let key = "media::".to_string() + media_path.to_str().unwrap();

        let cache = self.data_cache.read().unwrap();
        if cache.contains_key(&key) {
            cache.get(&key).unwrap().data.clone()
        } else {
            let data = fs::read(media_path).unwrap();
            let cache_entry = CacheEntry {
                id: CACHE_SEQ.fetch_add(1, Ordering::Relaxed),
                data: data.clone(),
            };
            drop(cache);
            {
                self.data_cache
                    .write()
                    .unwrap()
                    .insert(key.clone(), cache_entry);
            }
            self.cleanup();
            data
        }
    }

    pub fn read_cache(&self, cache_key: &str) -> Option<Vec<u8>> {
        self.data_cache
            .read()
            .unwrap()
            .get(cache_key)
            .map(|x| x.data.clone())
    }

    pub fn ref_cache_cast_array<T: Sized>(&self, cache_key: &str) -> Option<&[T]> {
        println!("read {cache_key}");
        self.data_cache.read().unwrap().get(cache_key).map(|x| {
            let cloned = x.data.clone();
            let ptr = cloned.as_ptr();
            let cast = ptr as *const T;
            unsafe { slice::from_raw_parts(cast, cloned.len() / size_of::<T>()) }
        })
    }

    pub fn precache(&self, cache_key: &str, data: Vec<u8>) {
        self.data_cache.write().unwrap().insert(
            cache_key.to_string(),
            CacheEntry {
                id: CACHE_SEQ.fetch_add(1, Ordering::Relaxed),
                data,
            },
        );
    }

    pub fn precache_cast<T: Sized>(&self, cache_key: &str, data: Vec<T>) {
        println!("precache {cache_key}");
        let ptr = data.as_ptr();
        let cast = ptr as *const u8;
        let slice = unsafe { slice::from_raw_parts(cast, data.len() * size_of::<T>()) };
        let mut vector = Vec::new();
        vector.extend_from_slice(slice);
        self.data_cache.write().unwrap().insert(
            cache_key.to_string(),
            CacheEntry {
                id: CACHE_SEQ.fetch_add(1, Ordering::Relaxed),
                data: vector,
            },
        );
    }

    pub fn remove_entry(&self, cache_key: &str) {
        println!("remove {cache_key}");
        self.data_cache.write().unwrap().remove(cache_key);
    }

    pub fn cleanup(&self) {
        while self.count_size() > CACHE_SIZE_LIMIT {
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
        cache
            .iter()
            .fold(0, |p, (_, entry)| entry.data.len() as u64 + p)
    }
}
