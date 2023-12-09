use crate::VecBTreeMap;
use core::ops::Deref;

extern crate std;

use std::dbg;

#[test]
fn stays_sorted() {
    let mut map = VecBTreeMap::with_capacity(3);
    map.insert("hi", 1.0);
    map.insert("mid", 5.0);
    map.insert("bye", -7.0);

    dbg!(&map);

    assert_eq!(map.deref(), &[("bye", -7.0), ("hi", 1.0), ("mid", 5.0)]);
    assert_eq!(map.insert("hi", 10.3), Some(1.0));
    assert_eq!(map.deref(), &[("bye", -7.0), ("hi", 10.3), ("mid", 5.0)]);
    assert_eq!(map.remove("hi"), Some(10.3));
    assert_eq!(map.deref(), &[("bye", -7.0), ("mid", 5.0)]);
}
