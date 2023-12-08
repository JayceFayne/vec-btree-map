#![no_std]
extern crate alloc;

mod deref;
mod index;
mod iter;
#[cfg(test)]
mod tests;

use alloc::vec::Vec;
use core::borrow::Borrow;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VecBTreeMap<K, V> {
    base: Vec<(K, V)>,
}

impl<K, V> VecBTreeMap<K, V> {
    #[inline]
    pub const fn new() -> Self {
        Self { base: Vec::new() }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            base: Vec::with_capacity(capacity),
        }
    }
}

impl<K, V> VecBTreeMap<K, V>
where
    K: Ord,
{
    #[inline]
    pub fn binary_search<Q: ?Sized>(&self, k: &Q) -> Result<usize, usize>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.base.binary_search_by(|e| e.0.borrow().cmp(k))
    }

    #[inline]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        match self.binary_search(&k) {
            Ok(i) => Some(core::mem::replace(&mut self.base[i].1, v)),
            Err(i) => {
                self.base.insert(i, (k, v));
                None
            }
        }
    }

    #[inline]
    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.binary_search(k).map(|i| self.base.remove(i).1).ok()
    }
}
