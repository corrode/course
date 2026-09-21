/// Finds all files with ".rs" extension.
///
/// As in the previous exercise, you'll filter the input, but here it's a
/// `&[&str]` (a borrowed slice of borrowed strings), so the iterator yields
/// `&&str`. Return owned `String`s so the caller can keep the results
/// independently of the input. That lets us focus on the iterator chain without
/// adding lifetime annotations. To go from `&&str` to `String`, use
/// [`str::to_string`].
fn find_rust_files(files: &[&str]) -> Vec<String> {
    todo!("Use an iterator pipeline to return .rs paths as owned Strings in input order")
}

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
