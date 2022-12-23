use std::fmt::Error;
use crate::allocator::alloc_manager::{AllocManager};

pub trait CachePolicy<K, V> {
    fn new<T>(alloc: &AllocManager, options: Option<T>) -> Self;
    fn destroy(&mut self, alloc: &AllocManager);
    // TODO: Create custom error
    fn request(&mut self, key: K, value: V) -> (Option<Error>, Option<V>);
    fn get(&mut self, alloc: &AllocManager, key: K) -> (V, Option<V>);
    fn contains(&self, key: K) -> bool;
    fn is_full(&self) -> bool;
}

pub struct Cache<K, V, P: CachePolicy<K, V>> {
    alloc: AllocManager,
    policy: dyn P,
}

impl<K, V, P: CachePolicy<K, V>> Cache<K, V, P> {

    pub fn new<T>(heap_size: usize, options: Option<T>) -> Self {
        let alloc: AllocManager = AllocManager::new(heap_size);
        alloc.0.
        Cache {
            alloc,
            policy: P::new(&alloc, options),
        }
    }

}