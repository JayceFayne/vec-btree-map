use core::fmt::{Debug, Formatter, Result};
use core::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator, Iterator};
use core::slice;

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Default)]
pub struct Iter<'a, K, V> {
    base: slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> Iter<'a, K, V> {
    #[inline]
    pub(super) fn new(base: slice::Iter<'a, (K, V)>) -> Self {
        Self { base }
    }
}

impl<K, V> Clone for Iter<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.base.clone())
    }
}

impl<'a, K: Debug, V: Debug> Debug for Iter<'a, K, V> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.base.next().map(|e| (&e.0, &e.1))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.base.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.base.nth(n).map(|e| (&e.0, &e.1))
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.base.last().map(|e| (&e.0, &e.1))
    }

    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.base.fold(init, |b, (k, v)| f(b, (k, v)))
    }
}

impl<K, V> DoubleEndedIterator for Iter<'_, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.base.next_back().map(|e| (&e.0, &e.1))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.base.nth_back(n).map(|e| (&e.0, &e.1))
    }
}

impl<K, V> ExactSizeIterator for Iter<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }
}

impl<K, V> FusedIterator for Iter<'_, K, V> {}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Default)]
pub struct IterMut<'a, K, V> {
    base: slice::IterMut<'a, (K, V)>,
}

impl<'a, K, V> IterMut<'a, K, V> {
    #[inline]
    pub(super) fn new(base: slice::IterMut<'a, (K, V)>) -> Self {
        Self { base }
    }
}

impl<'a, K: Debug, V: Debug> Debug for IterMut<'a, K, V> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list()
            .entries(Iter::new(self.base.as_slice().iter()))
            .finish()
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.base.next().map(|e| (&e.0, &mut e.1))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.base.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.base.nth(n).map(|e| (&e.0, &mut e.1))
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.base.last().map(|e| (&e.0, &mut e.1))
    }

    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.base.fold(init, |b, (k, v)| f(b, (k, v)))
    }
}

impl<K, V> DoubleEndedIterator for IterMut<'_, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.base.next_back().map(|e| (&e.0, &mut e.1))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.base.nth_back(n).map(|e| (&e.0, &mut e.1))
    }
}

impl<K, V> ExactSizeIterator for IterMut<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }
}

impl<K, V> FusedIterator for IterMut<'_, K, V> {}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Default)]
pub struct Keys<'a, K, V> {
    base: slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> Keys<'a, K, V> {
    #[inline]
    pub(super) const fn new(base: slice::Iter<'a, (K, V)>) -> Self {
        Self { base }
    }
}

impl<K, V> Clone for Keys<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.base.clone())
    }
}

impl<'a, K: Debug, V: Debug> Debug for Keys<'a, K, V> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.base.next().map(|e| &e.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.base.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.base.nth(n).map(|e| &e.0)
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.base.last().map(|e| &e.0)
    }

    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.base.fold(init, |b, e| f(b, &e.0))
    }
}

impl<K, V> DoubleEndedIterator for Keys<'_, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.base.next_back().map(|e| &e.0)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.base.nth_back(n).map(|e| &e.0)
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
#[derive(Default)]
pub struct Values<'a, K, V> {
    base: slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> Values<'a, K, V> {
    #[inline]
    pub(super) const fn new(base: slice::Iter<'a, (K, V)>) -> Self {
        Self { base }
    }
}

impl<K, V> Clone for Values<'_, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.base.clone())
    }
}

impl<'a, K: Debug, V: Debug> Debug for Values<'a, K, V> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.base.next().map(|e| &e.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.base.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.base.nth(n).map(|e| &e.1)
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.base.last().map(|e| &e.1)
    }

    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.base.fold(init, |b, e| f(b, &e.1))
    }
}

impl<K, V> DoubleEndedIterator for Values<'_, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.base.next_back().map(|e| &e.1)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.base.nth_back(n).map(|e| &e.1)
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
    base: slice::IterMut<'a, (K, V)>,
}

impl<'a, K, V> ValuesMut<'a, K, V> {
    #[inline]
    pub(super) const fn new(base: slice::IterMut<'a, (K, V)>) -> Self {
        Self { base }
    }
}

impl<'a, K: Debug, V: Debug> Debug for ValuesMut<'a, K, V> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list()
            .entries(Values::new(self.base.as_slice().iter()))
            .finish()
    }
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.base.next().map(|e| &mut e.1)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.base.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.base.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.base.nth(n).map(|e| &mut e.1)
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.base.last().map(|e| &mut e.1)
    }

    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.base.fold(init, |b, e| f(b, &mut e.1))
    }
}

impl<K, V> DoubleEndedIterator for ValuesMut<'_, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.base.next_back().map(|e| &mut e.1)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.base.nth_back(n).map(|e| &mut e.1)
    }
}

impl<K, V> ExactSizeIterator for ValuesMut<'_, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.base.len()
    }
}

impl<K, V> FusedIterator for ValuesMut<'_, K, V> {}
