/// Takes ownership of a String and modifies it.
/// When you pass a String to this function, ownership transfers.
fn take_ownership(s: String) -> String {
    let mut s = s;
    s.push_str(" - owned by Rust!");
    s
}

// Alternative:
//
//     fn take_ownership(s: String) -> String {
//         format!("{s} - owned by Rust!")
//     }
//
// This reuses `s` only to read it, and builds a brand-new String for the
// result. Under the hood `format!` expands to roughly:
//
//     let mut buf = String::new();
//     buf.write_fmt(format_args!("{s} - owned by Rust!")).unwrap();
//     buf
//
// So instead of growing the moved-in buffer in place (the `push_str`
// version), it allocates a fresh buffer and writes both pieces into it.
// The original `s` is dropped at the end of the function either way.

#[test]
fn test_take_ownership() {
    let s = String::from("Rust");
    let result = take_ownership(s);
    // Note: s is no longer valid here! It was moved.
    assert_eq!(result, "Rust - owned by Rust!");
}
