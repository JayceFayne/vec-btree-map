use crate::VecBTreeMap;
use core::ops::{Index, IndexMut};

impl<K, V> Index<usize> for VecBTreeMap<K, V> {
    type Output = V;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.base.index(i).1
    }
}

impl<K, V> IndexMut<usize> for VecBTreeMap<K, V> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.base.index_mut(i).1
    }
}
