use crate::VecBTreeMap;
use alloc::vec::Vec;
use core::ops::Deref;

impl<K, V> Deref for VecBTreeMap<K, V> {
    type Target = Vec<(K, V)>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
