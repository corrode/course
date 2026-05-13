use std::collections::HashMap;

/// Counts how many times each word appears.
/// Returns a HashMap mapping words to their counts.
///
/// The naive approach (`if contains_key(k) { *map.get_mut(k).unwrap() += 1 }
/// else { map.insert(k, 1) }`) works, but it does two lookups and fights
/// the borrow checker the moment you try to hold a reference into the map
/// while also calling `insert` on it. The standard idiom is
/// [`Entry::or_insert`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert):
///
/// ```ignore
/// *map.entry(key).or_insert(0) += 1;
/// ```
///
/// `entry(key)` reserves a single "slot" in the map for that key;
/// `or_insert(0)` either returns a mutable reference to the existing value
/// or inserts the default first and returns a reference to that.
/// See: <https://doc.rust-lang.org/std/collections/struct.HashMap.html>
fn count_words(words: &[&str]) -> HashMap<String, usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        let words = ["hello", "world", "hello", "rust"];
        let counts = count_words(&words);
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
        assert_eq!(counts.get("rust"), Some(&1));
    }
}
