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
mod status {
    enum State {
        Running,
        Stopped,
    }
}

/// TODO: Make this public and fix the calculator::add call
fn calculate(x: i32, y: i32) -> i32 {
    calculator::add(x, y)
}

/// TODO: Make this public and fix the config::Settings access
fn create_settings() -> config::Settings {
    config::Settings::new(8080)
}

/// TODO: Make this public and fix the status::State access  
fn get_status() -> status::State {
    status::State::Running
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
        let state = get_status();
        // Once you make State public, this will compile
    }
}
