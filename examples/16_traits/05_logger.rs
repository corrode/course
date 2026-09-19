/// One required method, with two defaults that build on it.
trait Logger {
    /// Turn a message into the final log line.
    fn log(&self, msg: &str) -> String;

    fn warn(&self, msg: &str) -> String {
        self.log(&format!("[WARN] {msg}"))
    }

    fn error(&self, msg: &str) -> String {
        self.log(&format!("[ERROR] {msg}"))
    }
}

/// A logger that returns the message untouched.
struct PlainLogger;

// Implement Logger for PlainLogger here. Only write log, returning the message
// as a String. Keep the default warn and error methods.

#[test]
fn log_returns_the_message_untouched() {
    let logger = PlainLogger;
    assert_eq!(Logger::log(&logger, "ready"), "ready");
    assert_eq!(Logger::log(&logger, "  still here  "), "  still here  ");
    assert_eq!(Logger::log(&logger, ""), "");
}

#[test]
fn warn_uses_the_default_prefix() {
    let logger = PlainLogger;
    assert_eq!(Logger::warn(&logger, "slow query"), "[WARN] slow query");
    assert_eq!(Logger::warn(&logger, ""), "[WARN] ");
}

#[test]
fn error_uses_the_default_prefix() {
    let logger = PlainLogger;
    assert_eq!(Logger::error(&logger, "disk full"), "[ERROR] disk full");
    assert_eq!(Logger::error(&logger, ""), "[ERROR] ");
}
