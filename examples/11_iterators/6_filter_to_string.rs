//! # Filter, then own the result
//!
//! Same shape as the previous step, but the input is a `&[&str]` (a
//! borrowed slice of borrowed strings), so the iterator yields `&&str`.
//! We sidestep that double-reference by returning owned `String`s; the
//! lesson here is iterators, not lifetimes.
//!
//! To go from `&&str` to `String`, reach for [`str::to_string`]. Chain
//! it after your `filter` with a `map`, then `collect` into a `Vec`.

/// Finds all files with ".rs" extension.
///
/// Same shape as the previous one, but the input is a `&[&str]` (a
/// borrowed slice of borrowed strings), so the iterator yields `&&str`.
/// We sidestep that double-reference by returning owned `String`s; the
/// lesson here is iterators, not lifetimes. To go from `&&str` to
/// `String`, reach for [`str::to_string`].
fn find_rust_files(files: &[&str]) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_rust_files() {
        let files = &[
            "main.rs",
            "README.md",
            "lib.rs",
            "package.json",
            "config.rs",
        ];
        let rust_files = find_rust_files(files);
        assert_eq!(rust_files, vec!["main.rs", "lib.rs", "config.rs"]);
    }
}
