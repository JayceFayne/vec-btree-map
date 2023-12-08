use core::fmt::{Debug, Formatter, Result};
use core::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator, Iterator};
use core::slice::{Iter, IterMut};

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Default, Clone)]
pub struct Keys<'a, K, V> {
    base: Iter<'a, (K, V)>,
}

impl<'a, K: Debug, V: Debug> Debug for Keys<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.base.fmt(f)
    }
}

impl<'a, K, V> Keys<'a, K, V> {
    #[inline]
    pub(crate) const fn new(base: Iter<'a, (K, V)>) -> Self {
        Self { base }
    }
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<&'a K> {
        self.base.next().map(|e| &e.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

impl<K, V> DoubleEndedIterator for Keys<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.base.next_back().map(|e| &e.0)
    }
}

impl<K, V> ExactSizeIterator for Keys<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }
}

impl<K, V> FusedIterator for Keys<'_, K, V> {}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Default, Clone)]
pub struct Values<'a, K, V> {
    base: Iter<'a, (K, V)>,
}

impl<'a, K: Debug, V: Debug> Debug for Values<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.base.fmt(f)
    }
}

impl<'a, K, V> Values<'a, K, V> {
    #[inline]
    pub(crate) const fn new(base: Iter<'a, (K, V)>) -> Self {
        Self { base }
    }
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<&'a V> {
        self.base.next().map(|e| &e.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

impl<K, V> DoubleEndedIterator for Values<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.base.next_back().map(|e| &e.1)
    }
}

impl<K, V> ExactSizeIterator for Values<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }
}

impl<K, V> FusedIterator for Values<'_, K, V> {}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Default)]
pub struct ValuesMut<'a, K, V> {
    base: IterMut<'a, (K, V)>,
}

impl<'a, K, V> ValuesMut<'a, K, V> {
    #[inline]
    pub(crate) const fn new(base: IterMut<'a, (K, V)>) -> Self {
        Self { base }
    }
}

impl<'a, K: Debug, V: Debug> Debug for ValuesMut<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.base.fmt(f)
    }
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<&'a mut V> {
        self.base.next().map(|e| &mut e.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }
}

impl<K, V> DoubleEndedIterator for ValuesMut<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.base.next_back().map(|e| &mut e.1)
    }
}

impl<K, V> ExactSizeIterator for ValuesMut<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }
}

impl<K, V> FusedIterator for ValuesMut<'_, K, V> {}
