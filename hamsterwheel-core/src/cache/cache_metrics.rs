use std::sync::atomic::{AtomicU64, Ordering};

pub trait CacheMetrics: Send + Sync {
    fn record_hit(&self);
    fn record_miss(&self);
    fn set_memory_usage_bytes(&self, bytes: u64);
}

pub struct InMemoryCacheMetrics {
    pub hits: AtomicU64,
    pub misses: AtomicU64,
    pub memory_usage_bytes: AtomicU64,
}

impl InMemoryCacheMetrics {
    pub fn new() -> Self {
        Self {
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            memory_usage_bytes: AtomicU64::new(0),
        }
    }
}

impl CacheMetrics for InMemoryCacheMetrics {
    fn record_hit(&self) {
        self.hits.fetch_add(1, Ordering::Relaxed);
    }

    fn record_miss(&self) {
        self.misses.fetch_add(1, Ordering::Relaxed);
    }

    fn set_memory_usage_bytes(&self, bytes: u64) {
        self.memory_usage_bytes.store(bytes, Ordering::Relaxed);
    }
}
