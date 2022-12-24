use std::fmt::Error;
use bumpalo::Bump;
use crate::allocator::alloc_manager::{AllocManager};

pub trait CachePolicy<K, V, T> {
    fn new(alloc: &Bump, options: Option<T>) -> Self;
    fn destroy(&mut self, alloc: &Bump);
    // TODO: Create custom error
    fn request(&mut self, key: K, value: V) -> (Option<Error>, Option<V>);
    fn get(&mut self, alloc: &Bump, key: K) -> (V, Option<V>);
    fn contains(&self, key: K) -> bool;
    fn is_full(&self) -> bool;
}

pub type KeyComparator<K, V> = fn(key1: K, key2: K, key2_value: V);

pub struct Cache<K, V, T, P: CachePolicy<K, V, T>> {
    alloc: Bump,
    policy: dyn P,
}

impl<K, V, T, P: CachePolicy<K, V, T>> Cache<K, V, T, P> {

    pub fn new(heap_size: usize, options: Option<T>) -> Self {
        let alloc: Bump = AllocManager::new(heap_size);
        Cache {
            alloc,
            policy: P::new(&alloc, options),
        }
    }

}