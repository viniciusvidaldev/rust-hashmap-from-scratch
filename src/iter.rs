use crate::map::HashMap;

pub struct Iter<'a, K, V> {
    map: &'a HashMap<K, V>,
    bucket: usize,
    at: usize,
}

impl<'a, K, V> Iter<'a, K, V> {
    fn new(map: &'a HashMap<K, V>) -> Self {
        Self {
            map,
            bucket: 0,
            at: 0,
        }
    }
}

impl<K, V> HashMap<K, V> {
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.into_iter()
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let bucket = self.map.buckets.get(self.bucket)?;
            match bucket.get(self.at) {
                Some((k, v)) => {
                    self.at += 1;
                    return Some((k, v));
                }
                None => {
                    self.at = 0;
                    self.bucket += 1;
                }
            }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_map_yields_nothing() {
        let map: HashMap<&str, i32> = HashMap::new();
        assert_eq!(map.iter().count(), 0);
    }

    #[test]
    fn yields_all_entries() {
        let mut map = HashMap::new();
        map.insert("foo", 1);
        map.insert("bar", 2);
        map.insert("baz", 3);
        assert_eq!(map.iter().count(), 3);
    }

    #[test]
    fn yields_key_value_pairs() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((&"foo", &42)));
    }

    #[test]
    fn yields_references_not_values() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        let (k, v) = map.iter().next().unwrap();
        assert_eq!(k, &"foo");
        assert_eq!(v, &42);
    }

    #[test]
    fn does_not_consume_map() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        let _ = map.iter().count();
        assert_eq!(map.get(&"foo"), Some(&42));
    }

    #[test]
    fn all_inserted_keys_are_visited() {
        let mut map = HashMap::new();
        map.insert("foo", 1);
        map.insert("bar", 2);
        map.insert("baz", 3);
        let mut keys: Vec<&&str> = map.iter().map(|(k, _)| k).collect();
        keys.sort();
        assert_eq!(keys, vec![&"bar", &"baz", &"foo"]);
    }
}
