//! # Modules and Visibility
//!
//! Modules organize code and control what's accessible. By default everything
//! is private. Use `pub` to make things public.
//!
//! This builds on structs and enums from previous exercises. The key insight:
//! Visibility is about designing clean APIs and hiding implementation details.

// Module with a function
mod calculator {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

// Module with a struct
mod config {
    struct Settings {
        port: u32,
    }

    impl Settings {
        fn new(port: u32) -> Self {
            Settings { port }
        }

        fn get_port(&self) -> u32 {
            self.port
        }
    }
}

// Module with an enum
// Make the enum variants comparable so the test below has something to assert on.
// (You'll still need to make `State` and its variants public to make `get_status`
// usable from outside the module.)
mod status {
    #[derive(Debug, PartialEq)]
    enum State {
        Running,
        Stopped,
    }
}

/// Wraps `calculator::add`. To make this compile, you need to make
/// `add` (in the `calculator` module above) public with `pub fn add`.
fn calculate(x: i32, y: i32) -> i32 {
    calculator::add(x, y)
}

#[test]
fn test_calculation() {
    assert_eq!(calculate(2, 3), 5);
}

/// Builds a `config::Settings`. To make this compile, you need to:
/// - make the `Settings` struct public,
/// - make `Settings::new` public,
/// - and (further down, for `test_settings`) make `get_port` public too.
/// Notice: `pub struct Settings` does not automatically make its
/// fields or methods public.
fn create_settings() -> config::Settings {
    config::Settings::new(8080)
}

#[test]
fn test_settings() {
    let settings = create_settings();
    assert_eq!(settings.get_port(), 8080);
}

/// Returns the current state. To make this compile, make the `State`
/// enum and its variants (`Running`, `Stopped`) public.
fn get_status() -> status::State {
    status::State::Running
}

#[test]
fn test_status() {
    let state = get_status();
    // Once you make `State` (and its variants) public, this compiles and
    // pins down the actual return value, not just "it builds".
    assert_eq!(state, status::State::Running);
}
