/// A small `Logger` trait with one required method (`log`) and two default methods (`warn` and `error`) that build on it.
///
/// You only have to implement `log`; you get `warn` and `error` for free unless you override them.
trait Logger {
    /// Required: turn a message into the final log line.
    fn log(&self, msg: &str) -> String;

    /// Default: prepend `"[WARN] "` and forward through `log`.
    fn warn(&self, msg: &str) -> String {
        self.log(&format!("[WARN] {msg}"))
    }

    /// Default: prepend `"[ERROR] "` and forward through `log`.
    fn error(&self, msg: &str) -> String {
        self.log(&format!("[ERROR] {msg}"))
    }
}

/// A logger that returns the message untouched.
/// Use the default `warn` and `error`; do *not* write them in this impl.
struct PlainLogger;

impl Logger for PlainLogger {
    /// Return `msg` as a `String`, with nothing added.
    fn log(&self, msg: &str) -> String {
        todo!()
    }
}

/// A logger that prepends a tag, like `"auth: something went wrong"`.
///
/// Use the default `warn`, so warnings come out as `"auth: [WARN] ..."`.
/// Override `error` to use a louder `[CRITICAL]` prefix instead of the default `[ERROR]`.
struct TaggedLogger {
    tag: String,
}

impl Logger for TaggedLogger {
    /// Return `"{tag}: {msg}"`.
    fn log(&self, msg: &str) -> String {
        todo!()
    }

    /// Build a `[CRITICAL]`-prefixed message and forward it through `log` so the tag still wraps the result.
    /// The expected output for `TaggedLogger { tag: "auth" }.error("nope")` is `"auth: [CRITICAL] nope"`.
    fn error(&self, msg: &str) -> String {
        todo!()
    }
}

#[test]
fn plain_logger_log_is_passthrough() {
    let l = PlainLogger;
    assert_eq!(l.log("ready"), "ready");
}

#[test]
fn plain_logger_inherits_warn() {
    let l = PlainLogger;
    assert_eq!(l.warn("slow query"), "[WARN] slow query");
}

#[test]
fn plain_logger_inherits_error() {
    let l = PlainLogger;
    assert_eq!(l.error("disk full"), "[ERROR] disk full");
}

#[test]
fn tagged_logger_prepends_tag() {
    let l = TaggedLogger {
        tag: "auth".to_string(),
    };
    assert_eq!(l.log("ok"), "auth: ok");
}

#[test]
fn tagged_logger_inherits_warn_via_its_own_log() {
    // Default warn calls self.log, so the tag wraps the [WARN] prefix.
    let l = TaggedLogger {
        tag: "auth".to_string(),
    };
    assert_eq!(l.warn("slow"), "auth: [WARN] slow");
}

#[test]
fn tagged_logger_overrides_error_with_critical() {
    let l = TaggedLogger {
        tag: "auth".to_string(),
    };
    assert_eq!(l.error("nope"), "auth: [CRITICAL] nope");
}
