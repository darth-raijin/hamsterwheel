use std::sync::atomic::AtomicU64;

pub struct CacheMetrics {
    pub hits: AtomicU64,
    pub misses: AtomicU64,
    pub inserts: AtomicU64,
    pub evictions: AtomicU64,
    pub removals: AtomicU64,
    pub memory_usage_bytes: AtomicU64,
}

