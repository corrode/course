// Module with a struct and a couple of methods.
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

/// Builds a `config::Settings` for callers that don't live inside
/// the `config` module.
fn create_settings() -> config::Settings {
    config::Settings::new(8080)
}

#[test]
fn test_create_settings() {
    let settings = create_settings();

    // `settings.port` would not compile from out here: the field is
    // private even once `Settings` is `pub`. Go through the accessor.
    assert_eq!(settings.get_port(), 8080);
}
