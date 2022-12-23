use std::fmt::Error;
use bumpalo::Bump;
use crate::allocator::alloc_manager::{AllocManager};

pub trait CachePolicy<K, V> {
    fn new<T>(alloc: &Bump, options: Option<T>) -> Self;
    fn destroy(&mut self, alloc: &Bump);
    // TODO: Create custom error
    fn request(&mut self, key: K, value: V) -> (Option<Error>, Option<V>);
    fn get(&mut self, alloc: &Bump, key: K) -> (V, Option<V>);
    fn contains(&self, key: K) -> bool;
    fn is_full(&self) -> bool;
}

pub struct Cache<K, V, P: CachePolicy<K, V>> {
    alloc: Bump,
    policy: dyn P,
}

impl<K, V, P: CachePolicy<K, V>> Cache<K, V, P> {

    pub fn new<T>(heap_size: usize, options: Option<T>) -> Self {
        let alloc: Bump = AllocManager::new(heap_size);
        Cache {
            alloc,
            policy: P::new(&alloc, options),
        }
    }

}