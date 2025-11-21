use std::collections::HashMap;
use crate::cache::cache_metrics::CacheMetrics;
use crate::cache::traits::Cache;
use std::hash::Hash;
use crate::cache::lru::node::Node;

pub struct LruConfig {
    pub capacity: usize,
    pub enable_metrics: bool,
}

pub struct LruCache<K, V> {
    map: HashMap<K, usize>,
    nodes: Vec<Node<K, V>>,
    head: Option<usize>,
    tail: Option<usize>,
    free_list: Vec<usize>,
    len: usize,
    capacity: usize,
    cache_metrics: Option<CacheMetrics>,
    lru_config: LruConfig,
}

impl<K, V> LruCache<K, V> {
    pub fn cache_metrics(&self) -> Option<&CacheMetrics> {
        todo!()
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

impl<K: Eq + Hash, V> Cache for LruCache<K, V> {
    type Key = K;
    type Value = V;

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

    fn metrics(&self) -> Option<&CacheMetrics> {
        todo!()
    }

    fn reset_metrics(&mut self) {
        todo!()
    }
}
