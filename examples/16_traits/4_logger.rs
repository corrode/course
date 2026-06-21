/// A small `Logger` trait. One required method (`log`), plus two
/// default methods that build on it (`warn` and `error`).
///
/// Implementors only have to write `log`; they get `warn` and
/// `error` for free unless they explicitly override them.
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

/// A logger that returns the message untouched. It should use the
/// default `warn` and `error` (do *not* write them in this impl).
struct PlainLogger;

impl Logger for PlainLogger {
    /// Return `msg` as a `String`, with nothing added.
    fn log(&self, msg: &str) -> String {
        todo!()
    }
}

/// A logger that prepends a tag, like `"auth: something went wrong"`.
///
/// It uses the default `warn` (so warnings come out as
/// `"auth: [WARN] ..."`), but *overrides* `error` to use a louder
/// `[CRITICAL]` prefix instead of the default `[ERROR]`.
struct TaggedLogger {
    tag: String,
}

impl Logger for TaggedLogger {
    /// Return `"{tag}: {msg}"`.
    fn log(&self, msg: &str) -> String {
        todo!()
    }

    /// Override: build a `[CRITICAL]`-prefixed message and forward
    /// it through `log` (so the tag still wraps the result).
    /// Expected output for `TaggedLogger { tag: "auth" }.error("nope")`
    /// is `"auth: [CRITICAL] nope"`.
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
