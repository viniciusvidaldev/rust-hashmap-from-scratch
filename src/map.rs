use std::{
    borrow::Borrow,
    hash::{DefaultHasher, Hash, Hasher},
};

const INITIAL_NBUCKETS: usize = 1;

pub struct HashMap<K, V> {
    pub(crate) buckets: Vec<Vec<(K, V)>>,
    pub(crate) len: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Extend<(K, V)> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<K, V> FromIterator<(K, V)> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = Self::new();
        map.extend(iter);
        map
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub(crate) fn bucket<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if self.buckets.is_empty() {
            return None;
        }

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        Some((hasher.finish() % self.buckets.len() as u64) as usize)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.len > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket = self.bucket(&key).expect("buckets non-empty after resize");
        let bucket = &mut self.buckets[bucket];

        for (k, v) in bucket.iter_mut() {
            if k == &key {
                return Some(std::mem::replace(v, value));
            }
        }

        self.len += 1;
        bucket.push((key, value));
        None
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.bucket(key)?;
        self.buckets[bucket]
            .iter()
            .find(|(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(key).is_some()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.bucket(key)?;
        let bucket = &mut self.buckets[bucket];
        let i = bucket.iter().position(|(k, _)| k.borrow() == key)?;
        self.len -= 1;
        Some(bucket.swap_remove(i).1)
    }

    pub(crate) fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };

        let mut new_buckets: Vec<Vec<(K, V)>> =
            std::iter::repeat_with(Vec::new).take(target_size).collect();

        for bucket in self.buckets.drain(..) {
            for (key, value) in bucket {
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                let new_bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
                new_buckets[new_bucket].push((key, value));
            }
        }

        self.buckets = new_buckets;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_map_is_empty() {
        let map: HashMap<&str, i32> = HashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn insert_increases_len() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());
    }

    #[test]
    fn insert_makes_key_retrievable() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert_eq!(map.get(&"foo"), Some(&42));
    }

    #[test]
    fn remove_returns_value() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert_eq!(map.remove(&"foo"), Some(42));
    }

    #[test]
    fn remove_makes_key_absent() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        map.remove(&"foo");
        assert_eq!(map.get(&"foo"), None);
    }

    #[test]
    fn remove_decreases_len() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        map.remove(&"foo");
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn get_absent_key_returns_none() {
        let map: HashMap<&str, i32> = HashMap::new();
        assert_eq!(map.get(&"foo"), None);
    }

    #[test]
    fn contains_key_returns_true_for_inserted_key() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert!(map.contains_key(&"foo"));
    }

    #[test]
    fn contains_key_returns_false_for_missing_key() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert!(!map.contains_key(&"bar"));
    }

    mod extend {
        use super::*;

        #[test]
        fn adds_pairs_to_empty_map() {
            let mut map = HashMap::new();
            map.extend([("a", 1), ("b", 2)]);
            assert_eq!(map.get(&"a"), Some(&1));
            assert_eq!(map.get(&"b"), Some(&2));
            assert_eq!(map.len(), 2);
        }

        #[test]
        fn overwrites_existing_keys() {
            let mut map = HashMap::new();
            map.insert("a", 1);
            map.extend([("a", 99), ("b", 2)]);
            assert_eq!(map.get(&"a"), Some(&99));
            assert_eq!(map.get(&"b"), Some(&2));
            assert_eq!(map.len(), 2);
        }

        #[test]
        fn accepts_any_into_iterator() {
            let mut map = HashMap::new();
            let pairs = vec![("a", 1), ("b", 2)];
            map.extend(pairs.into_iter().filter(|(_, v)| *v > 0));
            assert_eq!(map.len(), 2);
        }
    }

    mod from_iterator {
        use super::*;

        #[test]
        fn collects_pairs_into_map() {
            let map: HashMap<&str, i32> = [("a", 1), ("b", 2)].into_iter().collect();
            assert_eq!(map.get(&"a"), Some(&1));
            assert_eq!(map.get(&"b"), Some(&2));
            assert_eq!(map.len(), 2);
        }

        #[test]
        fn empty_iterator_yields_empty_map() {
            let map: HashMap<&str, i32> = std::iter::empty().collect();
            assert!(map.is_empty());
        }

        #[test]
        fn duplicate_keys_keep_last_value() {
            let map: HashMap<&str, i32> = [("a", 1), ("a", 2), ("a", 3)].into_iter().collect();
            assert_eq!(map.get(&"a"), Some(&3));
            assert_eq!(map.len(), 1);
        }
    }
}
