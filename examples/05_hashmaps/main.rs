//! # HashMaps
//!
//! Hash tables are one of the most important data structures ever invented.
//! They were first described by Hans Peter Luhn at IBM in the 1950s, and
//! today they power everything from database indexes to the caching that
//! makes web browsing fast.
//!
//! The "hash" in `HashMap` comes from hash functions: operations that turn
//! any input into a fixed-size number. That number tells the map which
//! bucket to look in, which is why lookups are (on average) constant time.

use std::collections::HashMap;

/// Creates a configuration map with default settings.
/// Returns a HashMap with "host" -> "localhost" and "port" -> "8080".
fn create_default_config() -> HashMap<String, String> {
    todo!()
}

#[test]
fn test_default_config() {
    let config = create_default_config();
    assert_eq!(config.get("host"), Some(&"localhost".to_string()));
    assert_eq!(config.get("port"), Some(&"8080".to_string()));
}

/// Updates a configuration value.
/// Inserts or updates the key-value pair.
///
/// One method on `HashMap` covers both cases; it doesn't care whether
/// the key was already there.
fn set_config_value(config: &mut HashMap<String, String>, key: &str, value: &str) {
    todo!()
}

#[test]
fn test_set_config() {
    let mut config = HashMap::new();
    set_config_value(&mut config, "debug", "true");
    assert_eq!(config.get("debug"), Some(&"true".to_string()));
}

/// Gets a configuration value by key.
/// Returns the value if found, "default" if not found.
///
/// Looking up a key returns an `Option<&V>`, because the key might not
/// be there. From chapter 7 you've already seen a few ways to collapse
/// an `Option` into a concrete value.
fn get_config_value(config: &HashMap<String, String>, key: &str) -> String {
    todo!()
}

#[test]
fn test_get_config() {
    let mut config = HashMap::new();
    config.insert("timeout".to_string(), "30".to_string());

    assert_eq!(get_config_value(&config, "timeout"), "30");
    assert_eq!(get_config_value(&config, "missing"), "default");
}

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

#[test]
fn test_word_counting() {
    let words = ["hello", "world", "hello", "rust"];
    let counts = count_words(&words);
    assert_eq!(counts.get("hello"), Some(&2));
    assert_eq!(counts.get("world"), Some(&1));
    assert_eq!(counts.get("rust"), Some(&1));
}
