use std::{
    collections::BTreeMap,
    fmt,
    hash::{DefaultHasher, Hash, Hasher},
    sync::{Arc, Mutex, PoisonError},
};

const SHARDS: usize = 16;

#[derive(Debug, Clone)]
pub struct ShardedMetrics {
    shards: Arc<Vec<Mutex<BTreeMap<String, i64>>>>,
}

impl ShardedMetrics {
    pub fn new() -> Self {
        Self {
            shards: Arc::new((0..SHARDS).map(|_| Mutex::default()).collect()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) {
        let key = key.into();
        let idx = shard_index(&key);
        let mut guard = self.shards[idx]
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        *guard.entry(key).or_insert(0) += 1;
    }
}

fn shard_index(key: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    usize::try_from(hasher.finish() % SHARDS as u64).unwrap_or(0)
}

impl fmt::Display for ShardedMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for shard in self.shards.iter() {
            let guard = shard.lock().unwrap_or_else(PoisonError::into_inner);
            for (key, value) in guard.iter() {
                writeln!(f, "{key}: {value}")?;
            }
        }
        Ok(())
    }
}

impl Default for ShardedMetrics {
    fn default() -> Self {
        Self::new()
    }
}
