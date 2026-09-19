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

struct TaggedLogger {
    tag: String,
}

impl Logger for TaggedLogger {
    fn log(&self, msg: &str) -> String {
        format!("{}: {msg}", self.tag)
    }

    // Add an error override here, using [CRITICAL] instead of [ERROR].
    // Forward through self.log so the tag appears exactly once.
}

#[test]
fn error_uses_critical_with_one_tag() {
    let logger = TaggedLogger {
        tag: "auth".to_string(),
    };
    assert_eq!(Logger::error(&logger, "nope"), "auth: [CRITICAL] nope");
}

#[test]
fn error_handles_other_tags_and_messages() {
    for (tag, msg) in [
        ("worker/7", "queue full"),
        ("db: replica", "  retry  "),
        ("", ""),
    ] {
        let logger = TaggedLogger {
            tag: tag.to_string(),
        };
        assert_eq!(
            Logger::error(&logger, msg),
            format!("{tag}: [CRITICAL] {msg}")
        );
    }
}

#[test]
fn overriding_error_leaves_warn_default() {
    let logger = TaggedLogger {
        tag: "cache".to_string(),
    };
    assert_eq!(Logger::error(&logger, "miss"), "cache: [CRITICAL] miss");
    assert_eq!(Logger::warn(&logger, "miss"), "cache: [WARN] miss");
    assert_eq!(Logger::warn(&logger, ""), "cache: [WARN] ");
}
