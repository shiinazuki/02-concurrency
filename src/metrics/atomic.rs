use std::{
    collections::HashMap,
    fmt,
    sync::{
        Arc,
        atomic::{AtomicI64, Ordering},
    },
};

use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct AtomicMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AtomicMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        Self {
            data: Arc::new(
                metric_names
                    .iter()
                    .map(|&name| (name, AtomicI64::new(0)))
                    .collect(),
            ),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| Error::UnknownMetric(key.to_owned()))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl fmt::Display for AtomicMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{key}: {}", value.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}
