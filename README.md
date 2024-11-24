# vec-btree-map &emsp; [![Action Badge]][actions] [![Version Badge]][crates.io] [![License Badge]][license]

[Version Badge]: https://img.shields.io/crates/v/vec-btree-map.svg
[crates.io]: https://crates.io/crates/vec-btree-map
[Action Badge]: https://github.com/JayceFayne/vec-btree-map/workflows/Rust/badge.svg
[actions]: https://github.com/JayceFayne/vec-btree-map/actions
[License Badge]: https://img.shields.io/crates/l/vec-btree-map.svg
[license]: https://github.com/JayceFayne/vec-btree-map/blob/master/LICENSE.md

Basically just a sorted [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) that can be used as a [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html).

## Why?

This library is meant for building data structures that you iterate over a lot.
Why not just use a HashMap? Iterating over a HashMap is slow and the iteration order is not guaranteed to be stable.

## Contributing

If you find any errors or just want to add a new feature feel free to [submit a PR](https://github.com/jaycefayne/vec-btree-map/pulls).
