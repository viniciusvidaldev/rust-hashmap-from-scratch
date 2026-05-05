use std::hash::Hash;

use crate::map::HashMap;

pub struct OccupiedEntry<'a, K, V> {
    entry: &'a mut (K, V),
}

pub struct VacantEntry<'a, K, V> {
    key: K,
    bucket: &'a mut Vec<(K, V)>,
}

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        let bucket = self.bucket(&key);
        let i = self.buckets[bucket].iter().position(|(k, _)| k == &key);

        match i {
            Some(i) => Entry::Occupied(OccupiedEntry {
                entry: &mut self.buckets[bucket][i],
            }),
            None => Entry::Vacant(VacantEntry {
                key,
                bucket: &mut self.buckets[bucket],
            }),
        }
    }
}

impl<'a, K, V> Entry<'a, K, V> {
    pub fn or_insert(self, value: V) -> &'a mut V {
        match self {
            Entry::Occupied(e) => &mut e.entry.1,
            Entry::Vacant(e) => {
                e.bucket.push((e.key, value));
                &mut e
                    .bucket
                    .last_mut()
                    .expect("bucket should not be empty after push")
                    .1
            }
        }
    }

    pub fn or_insert_with(self, f: impl FnOnce() -> V) -> &'a mut V {
        match self {
            Entry::Occupied(e) => &mut e.entry.1,
            Entry::Vacant(e) => {
                let value = f();
                e.bucket.push((e.key, value));
                &mut e
                    .bucket
                    .last_mut()
                    .expect("bucket should not be empty after push")
                    .1
            }
        }
    }
}

impl<'a, K, V> OccupiedEntry<'a, K, V> {
    pub fn get(&self) -> &V {
        &self.entry.1
    }

    pub fn get_mut(&mut self) -> &mut V {
        &mut self.entry.1
    }
}

impl<'a, K, V> VacantEntry<'a, K, V> {
    pub fn insert(self, value: V) -> &'a mut V {
        e.bucket.push((e.key, value));
        let (_, &mut value) = e
            .bucket
            .last_mut()
            .expect("bucket should not be empty after push");

        value
    }
    pub fn key(&self) -> &K {
        &self.key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod or_insert {
        use super::*;

        #[test]
        fn inserts_value_when_key_absent() {
            let mut map = HashMap::new();
            map.entry("foo").or_insert(42);
            assert_eq!(map.get(&"foo"), Some(&42));
        }

        #[test]
        fn does_not_overwrite_existing_value() {
            let mut map = HashMap::new();
            map.insert("foo", 1);
            map.entry("foo").or_insert(99);
            assert_eq!(map.get(&"foo"), Some(&1));
        }

        #[test]
        fn returns_mut_ref_to_value() {
            let mut map = HashMap::new();
            let v = map.entry("foo").or_insert(0);
            *v += 1;
            assert_eq!(map.get(&"foo"), Some(&1));
        }

        #[test]
        fn increments_len_only_when_absent() {
            let mut map = HashMap::new();
            map.entry("foo").or_insert(1);
            assert_eq!(map.len(), 1);
            map.entry("foo").or_insert(2);
            assert_eq!(map.len(), 1);
        }
    }

    mod or_insert_with {
        use super::*;

        #[test]
        fn inserts_value_from_closure_when_absent() {
            let mut map = HashMap::new();
            map.entry("foo").or_insert_with(|| 42);
            assert_eq!(map.get(&"foo"), Some(&42));
        }

        #[test]
        fn does_not_call_closure_when_occupied() {
            let mut map = HashMap::new();
            map.insert("foo", 1);
            let mut called = false;
            map.entry("foo").or_insert_with(|| {
                called = true;
                99
            });
            assert!(!called);
        }

        #[test]
        fn does_not_overwrite_existing_value() {
            let mut map = HashMap::new();
            map.insert("foo", 1);
            map.entry("foo").or_insert_with(|| 99);
            assert_eq!(map.get(&"foo"), Some(&1));
        }
    }
}
