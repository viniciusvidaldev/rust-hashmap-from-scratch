use hashmap::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // insert returns the previous value when the key already existed.
    assert!(scores.insert("alice", 10).is_none());
    assert!(scores.insert("bob", 20).is_none());
    assert!(scores.insert("carol", 30).is_none());
    assert_eq!(scores.insert("alice", 11), Some(10));

    println!("len   = {}", scores.len());
    println!("empty = {}", scores.is_empty());

    // get returns a reference into the bucket; the map keeps ownership.
    println!("alice = {:?}", scores.get(&"alice"));
    println!("dave  = {:?}", scores.get(&"dave"));

    println!("has bob   = {}", scores.contains_key(&"bob"));
    println!("has dave  = {}", scores.contains_key(&"dave"));

    // entry().or_insert returns a mut ref — useful for "insert if missing,
    // then mutate". Here we bump alice and insert dave with a default.
    *scores.entry("alice").or_insert(0) += 5;
    *scores.entry("dave").or_insert(0) += 1;

    // or_insert_with only runs the closure when the key is absent.
    scores.entry("erin").or_insert_with(|| 100);
    scores.entry("alice").or_insert_with(|| panic!("not called"));

    // Iteration order is unspecified — bucket layout depends on the hasher.
    println!("entries:");
    for (k, v) in &scores {
        println!("  {k} -> {v}");
    }

    // remove returns the value that was stored under the key.
    println!("remove bob = {:?}", scores.remove(&"bob"));
    println!("remove bob = {:?}", scores.remove(&"bob"));
    println!("len after removes = {}", scores.len());
}
