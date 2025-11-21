use std::hash::Hash;
use crate::cache::cache_metrics::CacheMetrics;

pub trait Cache {
    type Key: Eq + Hash;
    type Value;

    /// Lookup without taking ownership.
    ///
    /// For LRU-style implementations, `get` **SHOULD** mark the entry
    /// as recently used (bump it in the recency order).
    fn get(&mut self, key: &Self::Key) -> Option<&Self::Value>;

    /// Lookup without affecting recency / usage.
    ///
    /// For LRU-style implementations, `peek` **MUST NOT** change the
    /// recency order. This is useful for stats / introspection where
    /// you don't want to "touch" the entry.
    fn peek(&self, key: &Self::Key) -> Option<&Self::Value>;

    /// Mutable access to the value.
    ///
    /// For LRU-style implementations, `get_mut` **SHOULD** mark the
    /// entry as recently used.
    ///
    /// Useful if you want to update in-place without remove/put.
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    /// Mutable access without affecting recency / usage.
    ///
    /// For LRU-style implementations, `peek_mut` **MUST NOT** change
    /// the recency order. Only the value itself may be mutated.
    fn peek_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    /// Insert or update an entry.
    ///
    /// Returns the *previous* value if there was one, so callers can
    /// reuse or drop it explicitly.
    fn put(
        &mut self,
        key: Self::Key,
        value: Self::Value,
    ) -> Option<Self::Value>;

    /// Remove a key, if present, returning the owned value.
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;

    /// Clear the entire cache.
    fn clear(&mut self);

    /// Current number of entries in the cache.
    fn len(&self) -> usize;

    /// Maximum number of entries this cache will store.
    fn capacity(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn metrics(&self) -> Option<&CacheMetrics>;

    fn reset_metrics(&mut self);
}
