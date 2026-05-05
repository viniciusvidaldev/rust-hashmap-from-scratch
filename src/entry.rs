use std::hash::Hash;

use crate::map::HashMap;

pub struct OccupiedEntry<'a, K: 'a, V: 'a> {
    entry: &'a mut (K, V),
}

pub struct VacantEntry<'a, K: 'a, V: 'a> {
    key: K,
    bucket: &'a mut Vec<(K, V)>,
    len: &'a mut usize,
}

pub enum Entry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        if self.buckets.is_empty() || self.len > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket = self
            .bucket(&key)
            .expect("buckets non-empty after resize");

        match self.buckets[bucket].iter().position(|(k, _)| k == &key) {
            Some(i) => Entry::Occupied(OccupiedEntry {
                entry: &mut self.buckets[bucket][i],
            }),
            None => Entry::Vacant(VacantEntry {
                key,
                bucket: &mut self.buckets[bucket],
                len: &mut self.len,
            }),
        }
    }
}

impl<'a, K: 'a, V: 'a> OccupiedEntry<'a, K, V> {
    pub fn get(&self) -> &V {
        &self.entry.1
    }

    pub fn get_mut(&mut self) -> &mut V {
        &mut self.entry.1
    }
}

impl<'a, K: 'a, V: 'a> VacantEntry<'a, K, V> {
    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Hash + Eq,
    {
        *self.len += 1;
        self.bucket.push((self.key, value));
        &mut self.bucket.last_mut().unwrap().1
    }

    pub fn key(&self) -> &K {
        &self.key
    }
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: Hash + Eq,
{
    pub fn or_insert(self, value: V) -> &'a mut V {
        match self {
            Entry::Occupied(e) => &mut e.entry.1,
            Entry::Vacant(e) => e.insert(value),
        }
    }

    pub fn or_insert_with<F>(self, maker: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(e) => &mut e.entry.1,
            Entry::Vacant(e) => e.insert(maker()),
        }
    }

    pub fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        self.or_insert_with(Default::default)
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
