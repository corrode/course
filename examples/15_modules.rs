//! # Modules and Visibility
//!
//! Modules organize code and control what's accessible. By default everything
//! inside a module is private to that module. Use `pub` to make things visible
//! to the outside world.
//!
//! This builds on structs and enums from previous exercises. The key insight:
//! visibility is about designing clean APIs and hiding implementation details.
//!
//! In this exercise, the modules below already expose a small public API.
//! Your job is to implement the wrapper functions that use them, and to
//! observe how `pub` controls what callers can reach.
//!
//! See: <https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html>

// A module exposing a single public function.
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    // Private helper - not visible outside this module.
    #[allow(dead_code)]
    fn internal_double(x: i32) -> i32 {
        x * 2
    }
}

// A module exposing a struct with public constructors and accessors.
// Note that the `port` field stays private - callers must go through `get_port`.
mod config {
    pub struct Settings {
        port: u32,
    }

    impl Settings {
        pub fn new(port: u32) -> Self {
            Settings { port }
        }

        pub fn get_port(&self) -> u32 {
            self.port
        }
    }
}

// A module exposing an enum. Note: `pub enum` makes the variants public too.
mod status {
    pub enum State {
        Running,
        Stopped,
    }
}

/// Add two numbers using the `calculator` module.
fn calculate(x: i32, y: i32) -> i32 {
    todo!()
}

/// Build a `Settings` value with port 8080 using the `config` module.
fn create_settings() -> config::Settings {
    todo!()
}

/// Return `status::State::Running` using the `status` module.
fn get_status() -> status::State {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculation() {
        assert_eq!(calculate(2, 3), 5);
    }

    #[test]
    fn test_settings() {
        let settings = create_settings();
        assert_eq!(settings.get_port(), 8080);
    }

    #[test]
    fn test_status() {
        // Once `get_status` returns `State::Running`, this match should succeed.
        let state = get_status();
        assert!(matches!(state, status::State::Running));
    }
}
