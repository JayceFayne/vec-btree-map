#![no_std]
extern crate alloc;

mod deref;
mod index;
mod iter;
#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod tests;

use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{self, Debug, Formatter};
use core::mem;

pub use iter::{Iter, Keys, Values, ValuesMut};

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VecBTreeMap<K, V> {
    base: Vec<(K, V)>,
}

impl<K, V> VecBTreeMap<K, V> {
    /// Constructs a new, empty `VecBTreeMap<K, V>`.
    ///
    /// The map is initially created with a capacity of 0, so it will not allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(unused_mut)]
    /// use vec_btree_map::VecBTreeMap;
    ///
    /// let mut map: VecBTreeMap<String, f64> = VecBTreeMap::new();
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { base: Vec::new() }
    }

    /// Constructs a new, empty `VecBTreeMap<K, V>` with at least the specified capacity.
    ///
    /// The map will be able to hold at least `capacity` elements without
    /// reallocating. This method is allowed to allocate for more elements than
    /// `capacity`. If `capacity` is 0, the map will not allocate.
    ///
    /// It is important to note that although the returned map has the
    /// minimum *capacity* specified, the map will have a zero *length*. For
    /// an explanation of the difference between length and capacity, see
    /// *[Capacity and reallocation]*.
    ///
    /// If it is important to know the exact allocated capacity of a `VecBTreeMap<K, V>`,
    /// always use the [`capacity`] method after construction.
    ///
    /// [Capacity and reallocation]: Vec#capacity-and-reallocation
    /// [`capacity`]: Vec::capacity
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_btree_map::VecBTreeMap;
    ///
    /// let mut map = VecBTreeMap::with_capacity(10);
    ///
    /// // The map contains no items, even though it has capacity for more
    /// assert_eq!(map.len(), 0);
    /// assert!(map.capacity() >= 10);
    ///
    /// // These are all done without reallocating...
    /// for i in 0..10 {
    ///     map.insert(i, i);
    /// }
    /// assert_eq!(map.len(), 10);
    /// assert!(map.capacity() >= 10);
    ///
    /// // ...but this may make the map reallocate
    /// map.insert(11, 0);
    /// assert_eq!(map.len(), 11);
    /// assert!(map.capacity() >= 11);
    /// ```
    ///     
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            base: Vec::with_capacity(capacity),
        }
    }

    /// An iterator yielding all key-value paris from start to end.
    /// The iterator element type is `(&K, &V)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_btree_map::VecBTreeMap;
    ///
    /// let mut map = VecBTreeMap::with_capacity(3);
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// let mut iter = map.iter();
    ///
    /// for (key, value) in iter {
    ///     println!("{key}: {value}");
    /// }
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(self.base.iter())
    }

    /// An iterator yielding all keys from start to end.
    /// The iterator element type is `&K`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_btree_map::VecBTreeMap;
    ///
    /// let mut map = VecBTreeMap::with_capacity(3);
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    ///let mut keys = map.keys();
    ///
    /// assert_eq!(keys.next(), Some(&"a"));
    /// assert_eq!(keys.next(), Some(&"b"));
    /// assert_eq!(keys.next(), Some(&"c"));
    /// assert_eq!(keys.next(), None);
    /// ```
    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys::new(self.base.iter())
    }

    /// An iterator yielding all values from start to end.
    /// The iterator element type is `&V`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_btree_map::VecBTreeMap;
    ///
    /// let mut map = VecBTreeMap::with_capacity(3);
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    ///let mut keys = map.values();
    ///
    /// assert_eq!(keys.next(), Some(&1));
    /// assert_eq!(keys.next(), Some(&2));
    /// assert_eq!(keys.next(), Some(&3));
    /// assert_eq!(keys.next(), None);
    /// ```
    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        Values::new(self.base.iter())
    }

    /// An iterator yielding all values mutably from start to end.
    /// The iterator element type is `&V`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_btree_map::VecBTreeMap;
    ///
    /// let mut map = VecBTreeMap::with_capacity(3);
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for val in map.values_mut() {
    ///     *val *= *val;
    /// }
    ///
    ///let mut keys = map.values();
    ///
    /// assert_eq!(keys.next(), Some(&1));
    /// assert_eq!(keys.next(), Some(&4));
    /// assert_eq!(keys.next(), Some(&9));
    /// assert_eq!(keys.next(), None);
    /// ```
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut::new(self.base.iter_mut())
    }
}

impl<K, V> VecBTreeMap<K, V>
where
    K: Ord,
{
    /// Binary searches this map for a given key.
    ///
    /// If the key is found then [`Result::Ok`] is returned, containing the
    /// index of the matching key.
    /// If the key is not found then [`Result::Err`] is returned, containing
    /// the index where a matching key-value pair could be inserted while maintaining
    /// sorted order.
    ///
    /// # Examples
    ///
    /// Looks up a series of four elements.
    /// The first is found, the second and third are not.
    ///
    /// ```
    /// use vec_btree_map::VecBTreeMap;
    ///
    /// let mut map = VecBTreeMap::with_capacity(3);
    /// map.insert("a", 1);
    /// map.insert("c", 2);
    /// map.insert("d", 3);
    ///
    /// assert_eq!(map.binary_search("a"), Ok(0));
    /// assert_eq!(map.binary_search("b"), Err(1));
    /// assert_eq!(map.binary_search("e"), Err(3));
    /// ```
    #[inline]
    pub fn binary_search<Q: ?Sized>(&self, k: &Q) -> Result<usize, usize>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.base.binary_search_by(|e| e.0.borrow().cmp(k))
    }

    /// Appends a key-value pair to the back of the map.
    ///
    /// If the map woudn't be sorted anymore by appending
    /// the key-value pair to the back of the map, [`Some`]`(K, V)` is returned.
    /// Otherwise [`None`] is returned.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    #[inline]
    pub fn push(&mut self, k: K, v: V) -> Option<(K, V)> {
        let last = self.len().saturating_sub(1);
        if let Some((key, _)) = self.get(last) {
            if key >= &k {
                return Some((k, v));
            }
        }
        self.base.push((k, v));
        None
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_btree_map::VecBTreeMap;
    ///
    /// let mut map = VecBTreeMap::new();
    ///
    /// assert_eq!(map.is_empty(), true);
    /// assert_eq!(map.insert("a", 1), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert("a", 2);
    /// assert_eq!(map.insert("a", 3), Some(2));
    /// assert_eq!(map[0], 3);
    /// ```
    #[inline]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        match self.binary_search(&k) {
            Ok(i) => Some(mem::replace(&mut self.base[i].1, v)),
            Err(i) => {
                self.base.insert(i, (k, v));
                None
            }
        }
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Ord`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_btree_map::VecBTreeMap;
    ///
    /// let mut map = VecBTreeMap::new();
    /// map.insert("a", 1);
    /// assert_eq!(map.remove("a"), Some(1));
    /// assert_eq!(map.remove("a"), None);
    /// ```
    #[inline]
    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.binary_search(k).map(|i| self.base.remove(i).1).ok()
    }

    /// Removes the last key-value pair from the map and returns it, or [`None`] if it
    /// is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_btree_map::VecBTreeMap;
    ///
    /// let mut map = VecBTreeMap::new();
    /// map.insert("a", 1);
    /// assert_eq!(map.pop(), Some(("a", 1)));
    /// assert_eq!(map.pop(), None);
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.base.pop()
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory
    /// for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_btree_map::VecBTreeMap;
    ///
    /// let mut a = VecBTreeMap::new();
    /// a.insert(1, "a");
    /// a.clear();
    /// assert!(a.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.base.clear()
    }
}

impl<K: Clone, V: Clone> Clone for VecBTreeMap<K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
        }
    }
}

impl<K: Debug, V: Debug> Debug for VecBTreeMap<K, V> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
