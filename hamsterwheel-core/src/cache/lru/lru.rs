use crate::cache::cache_metrics::CacheMetrics;
use crate::cache::lru::node::Node;
use crate::cache::traits::Cache;
use std::collections::HashMap;
use std::hash::Hash;

pub const ERR_ZERO_CAPACITY: &str = "LruCache capacity must be greater than 0";

pub struct LruConfig {
    pub capacity: usize,
    pub enable_metrics: bool,
}

pub struct LruCache<K, V, M>
where
    M: CacheMetrics,
{
    map: HashMap<K, usize>,
    nodes: Vec<Node<K, V>>,
    head: Option<usize>,
    tail: Option<usize>,
    free_list: Vec<usize>,
    len: usize,
    capacity: usize,
    cache_metrics: Option<M>,
    lru_config: LruConfig,
}

impl<K, V, M> LruCache<K, V, M>
where
    M: CacheMetrics,
{
    pub fn cache_metrics(&self) -> Option<&M> {
        self.cache_metrics.as_ref()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Grow internal storage to fit at least
    /// `self.len() + additional` entries without further reallocation.
    pub fn reserve(&mut self, additional: usize) {
        todo!()
    }

    /// Try to shrink internal storage to fit the current number of
    /// entries as tightly as possible.
    ///
    /// Should be avoided on hot path.
    pub fn shrink_to_fit(&mut self) {
        todo!()
    }

    pub fn set_capacity(&mut self, new_capacity: usize) {
        todo!()
    }
}

impl<K, V, M> Cache for LruCache<K, V, M>
where
    K: Eq + Hash,
    M: CacheMetrics,
{
    type Key = K;
    type Value = V;
    type Metrics = M;

    fn get(&mut self, key: &Self::Key) -> Option<&Self::Value> {
        todo!()
    }

    fn peek(&self, key: &Self::Key) -> Option<&Self::Value> {
        todo!()
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        todo!()
    }

    fn peek_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        todo!()
    }

    fn put(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        todo!()
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn capacity(&self) -> usize {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn metrics(&self) -> Option<&Self::Metrics> {
        todo!()
    }

    fn reset_metrics(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::cache_metrics::InMemoryCacheMetrics;

    // Focus: the raw struct + inherent impl (new, capacity, cache_metrics)
    mod lru_struct {
        use super::*;

        #[test]
        #[should_panic(expected = "capacity must be greater than 0")]
        fn new_with_zero_capacity_panics() {}

        #[test]
        fn can_create_without_metrics_backend() {}

        #[test]
        fn can_create_with_metrics_backend() {}
    }

    mod lru_cache_impl {
        use super::*;
    }
}
