/// A rule returns `Ok(())` on success or `Err(message)` on failure.
trait Validator {
    fn check(&self, input: &str) -> Result<(), String>;
}

/// Require a case-sensitive substring.
/// On failure, return `must contain '<needle>'` with the configured needle.
struct MustContain {
    needle: String,
}

// Implement Validator for MustContain here.

/// Reject a case-sensitive substring.
/// On failure, return `must not contain '<forbidden>'` with the configured text.
struct MustNotContain {
    forbidden: String,
}

// Implement Validator for MustNotContain here.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_substring() {
        let rule = MustContain {
            needle: "cat".to_string(),
        };
        assert_eq!(Validator::check(&rule, "a cat naps"), Ok(()));
        assert_eq!(Validator::check(&rule, "cat"), Ok(()));
        for input in ["dog", "CAT", ""] {
            assert_eq!(
                Validator::check(&rule, input),
                Err("must contain 'cat'".to_string())
            );
        }
    }

    #[test]
    fn forbidden_substring() {
        let rule = MustNotContain {
            forbidden: "cat".to_string(),
        };
        for input in ["a cat naps", "cat"] {
            assert_eq!(
                Validator::check(&rule, input),
                Err("must not contain 'cat'".to_string())
            );
        }
        for input in ["dog", "CAT", ""] {
            assert_eq!(Validator::check(&rule, input), Ok(()));
        }
    }

    #[test]
    fn empty_patterns_follow_contains() {
        let required = MustContain {
            needle: String::new(),
        };
        let forbidden = MustNotContain {
            forbidden: String::new(),
        };
        for input in ["", "anything"] {
            assert_eq!(Validator::check(&required, input), Ok(()));
            assert_eq!(
                Validator::check(&forbidden, input),
                Err("must not contain ''".to_string())
            );
        }
    }
}
