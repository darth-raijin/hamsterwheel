use std::time::Instant;

pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    pub prev: Option<usize>,
    pub next: Option<usize>,
    pub expires_at: Option<Instant>,
}