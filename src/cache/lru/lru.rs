use std::fmt::Error;
use bumpalo::Bump;
use crate::cache::cache::CachePolicy;
use crate::allocator::alloc_manager::AllocManager;

pub struct LRUPolicy<K, V> {
    size: usize,
}

impl<K, V> CachePolicy<K, V> for LRUPolicy<K, V> {
    fn new<T>(alloc: &Bump, options: Option<T>) -> &Self {
        return AllocManager::handled_alloc(bump, LRUPolicy { size: 0 });
    }

    fn destroy(&mut self, alloc: &Bump) {
        todo!()
    }

    fn request(&mut self, key: K, value: V) -> (Option<Error>, Option<V>) {
        todo!()
    }

    fn get(&mut self, alloc: &Bump, key: K) -> (V, Option<V>) {
        todo!()
    }

    fn contains(&self, key: K) -> bool {
        todo!()
    }

    fn is_full(&self) -> bool {
        todo!()
    }
}