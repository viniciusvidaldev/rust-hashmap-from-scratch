use std::hash::{DefaultHasher, Hash, Hasher};

const INITIAL_NBUCKETS: usize = 1;

pub struct HashMap<K, V> {
    pub(crate) buckets: Vec<Vec<(K, V)>>,
    len: usize,
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

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub(crate) fn bucket(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.len > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];

        self.len += 1;
        for (k, v) in bucket.iter_mut() {
            if k == &key {
                return Some(std::mem::replace(v, value));
            }
        }

        bucket.push((key, value));
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if self.is_empty() {
            return None;
        }

        let bucket = self.bucket(key);
        self.buckets[bucket]
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket = self.bucket(key);
        let bucket = &mut self.buckets[bucket];
        let i = bucket.iter().position(|(k, _)| k == key)?;
        self.len -= 1;
        Some(bucket.swap_remove(i).1)
    }

    fn resize(&mut self) {
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
}
