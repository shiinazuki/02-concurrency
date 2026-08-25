use std::{
    collections::BTreeMap,
    fmt,
    sync::{Arc, Mutex, PoisonError},
};

#[derive(Debug, Clone, Default)]
pub struct LockedMetrics {
    data: Arc<Mutex<BTreeMap<String, i64>>>,
}

impl LockedMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc(&self, key: impl Into<String>) {
        let mut guard = self.data.lock().unwrap_or_else(PoisonError::into_inner);
        *guard.entry(key.into()).or_insert(0) += 1;
    }
}

impl fmt::Display for LockedMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = self.data.lock().unwrap_or_else(PoisonError::into_inner);
        for (key, value) in guard.iter() {
            writeln!(f, "{key}: {value}")?;
        }
        Ok(())
    }
}
