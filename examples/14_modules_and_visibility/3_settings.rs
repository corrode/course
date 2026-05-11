// Module with a struct.
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

/// Builds a `config::Settings`. To make this compile, you need to:
/// - make the `Settings` struct public,
/// - make `Settings::new` public,
/// - and (further down, for `test_create_settings`) make `get_port` public too.
///
/// Notice: `pub struct Settings` does not automatically make its
/// fields or methods public.
fn create_settings() -> config::Settings {
    config::Settings::new(8080)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_settings() {
        let settings = create_settings();
        assert_eq!(settings.get_port(), 8080);
    }
}
