use std::fmt::Error;
use crate::allocator::alloc_manager::{AllocManager};

pub trait CachePolicy<T, K, V> {
    fn new(alloc: &AllocManager, size: usize, options: Option<T>) -> Self;
    fn destroy(&mut self, alloc: &AllocManager);
    // TODO: Create custom error
    fn request(&mut self, key: K, value: V) -> (Option<Error>, Option<V>);
    fn get(&mut self, alloc: &AllocManager, key: K) -> (V, Option<V>);
    fn contains(&self, key: K) -> bool;
    fn is_full(&self) -> bool;

}

pub struct Cache {
}