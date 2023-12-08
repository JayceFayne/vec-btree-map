#![no_std]
extern crate alloc;

mod deref;
mod index;
mod iter;
#[cfg(test)]
mod tests;

use alloc::vec::{IntoIter, Vec};
use core::borrow::Borrow;

pub use iter::{Keys, Values, ValuesMut};

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

    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys::new(self.base.iter())
    }

    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        Values::new(self.base.iter())
    }

    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut::new(self.base.iter_mut())
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

impl<K, V> IntoIterator for VecBTreeMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter()
    }
}
