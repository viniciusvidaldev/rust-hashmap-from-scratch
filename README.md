# hashmap

A toy `HashMap<K, V>` written from scratch in Rust as a study exercise.

It uses separate chaining — `Vec<Vec<(K, V)>>` — and grows when the load
factor passes 3/4, doubling the bucket count and rehashing every entry.
Hashing goes through `std::hash::DefaultHasher`.

This is **not** a replacement for `std::collections::HashMap`. It exists
to make the moving parts visible: bucket selection, collision handling,
resizing, the `Entry` API, and writing a custom `Iterator` /
`IntoIterator`.

## Layout

- `src/lib.rs` — module declarations and public re-exports
- `src/map.rs`   — `HashMap` core (`new`, `insert`, `get`, `remove`, …) + tests
- `src/iter.rs`  — `Iter` and `IntoIterator for &HashMap` + tests
- `src/entry.rs` — `Entry` / `OccupiedEntry` / `VacantEntry` and `HashMap::entry` + tests
- `src/main.rs`  — demo binary that exercises the public API

Tests live next to the code they cover, gated by `#[cfg(test)]`.

## Run

```sh
cargo run     # runs the demo in src/main.rs
cargo test    # runs the unit tests across map/iter/entry
```

## API

| Method                       | Returns           | Notes                                    |
| ---------------------------- | ----------------- | ---------------------------------------- |
| `HashMap::new()` / `default()` | `HashMap<K, V>` | Starts with zero buckets                 |
| `insert(key, value)`         | `Option<V>`       | Returns the old value if the key existed |
| `get(&key)`                  | `Option<&V>`      | `key` is `&Q` where `K: Borrow<Q>` — so a `HashMap<String, _>` accepts `&str` lookups |
| `contains_key(&key)`         | `bool`            | Same `Borrow<Q>` flexibility as `get`    |
| `remove(&key)`               | `Option<V>`       | `swap_remove` inside the bucket          |
| `len()` / `is_empty()`       | `usize` / `bool`  |                                          |
| `entry(key)`                 | `Entry<'_, K, V>` | `or_insert` / `or_insert_with` / `or_default`; match on the variants for `OccupiedEntry::{get, get_mut}` and `VacantEntry::{key, insert}` |
| `iter()` / `&map` in a loop  | `Iter<'_, K, V>`  | Order is unspecified                     |
| `extend(pairs)`              | —                 | Bulk insert from any `IntoIterator<Item = (K, V)>` |
| `pairs.collect()`            | `HashMap<K, V>`   | Build a map directly via `FromIterator`  |

`K` must be `Hash + Eq` for any operation that needs to locate a bucket.

## Example

```rust
use hashmap::HashMap;

// Build directly from pairs via FromIterator.
let mut scores: HashMap<&str, i32> = [("alice", 10), ("bob", 20)].into_iter().collect();

assert_eq!(scores.get(&"alice"), Some(&10));
assert_eq!(scores.insert("alice", 11), Some(10));

// Insert-if-missing-then-mutate via the Entry API.
*scores.entry("alice").or_insert(0) += 5;
*scores.entry("dave").or_default() += 1;
scores.entry("erin").or_insert_with(|| 100);

// Bulk insert.
scores.extend([("frank", 7), ("gina", 8)]);

for (k, v) in &scores {
    println!("{k} -> {v}");
}
```
