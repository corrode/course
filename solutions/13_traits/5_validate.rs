/// A composable validation rule. `Ok(())` means the input is fine
/// for this rule; `Err(message)` explains what's wrong.
trait Validator {
    fn check(&self, input: &str) -> Result<(), String>;
}

/// Rule: the input must be at least `n` characters long.
///
/// Already implemented for you as the worked example. Read it,
/// then write the other two impls in the same way.
struct MinLength {
    n: usize,
}

impl Validator for MinLength {
    fn check(&self, input: &str) -> Result<(), String> {
        if input.chars().count() < self.n {
            Err(format!("must be at least {} characters", self.n))
        } else {
            Ok(())
        }
    }
}

/// Rule: the input must contain `needle` as a substring.
///
/// On failure return `Err(format!("must contain '{}'", self.needle))`.
struct MustContain {
    needle: String,
}

impl Validator for MustContain {
    fn check(&self, input: &str) -> Result<(), String> {
        if input.contains(&self.needle) {
            Ok(())
        } else {
            Err(format!("must contain '{}'", self.needle))
        }
    }
}

/// Rule: the input must *not* contain `forbidden` as a substring.
///
/// On failure return `Err(format!("must not contain '{}'", self.forbidden))`.
struct MustNotContain {
    forbidden: String,
}

impl Validator for MustNotContain {
    fn check(&self, input: &str) -> Result<(), String> {
        if input.contains(&self.forbidden) {
            Err(format!("must not contain '{}'", self.forbidden))
        } else {
            Ok(())
        }
    }
}

/// Run every validator against `input` and collect the failure
/// messages in the order the validators appear.
///
/// The slice element type is `&dyn Validator`: a reference to a
/// trait object. The slice can mix `MinLength`, `MustContain`, and
/// `MustNotContain` (and any future implementor) freely. That's the
/// whole point of trait objects.
///
/// An input that passes everything returns an empty `Vec`. The
/// returned `Vec<String>` contains only the `Err` messages.
fn collect_errors(validators: &[&dyn Validator], input: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for validator in validators {
        if let Err(message) = validator.check(input) {
            errors.push(message);
        }
    }
    errors
}

#[test]
fn min_length_passes_when_long_enough() {
    let v = MinLength { n: 3 };
    assert_eq!(v.check("abcd"), Ok(()));
}

#[test]
fn min_length_fails_when_too_short() {
    let v = MinLength { n: 5 };
    assert_eq!(
        v.check("hi"),
        Err("must be at least 5 characters".to_string())
    );
}

#[test]
fn must_contain_passes() {
    let v = MustContain {
        needle: "@".to_string(),
    };
    assert_eq!(v.check("alice@example.com"), Ok(()));
}

#[test]
fn must_contain_fails() {
    let v = MustContain {
        needle: "@".to_string(),
    };
    assert_eq!(v.check("alice"), Err("must contain '@'".to_string()));
}

#[test]
fn must_not_contain_passes() {
    let v = MustNotContain {
        forbidden: " ".to_string(),
    };
    assert_eq!(v.check("no-spaces"), Ok(()));
}

#[test]
fn must_not_contain_fails() {
    let v = MustNotContain {
        forbidden: " ".to_string(),
    };
    assert_eq!(
        v.check("has a space"),
        Err("must not contain ' '".to_string())
    );
}

#[test]
fn collect_errors_on_clean_input() {
    let r1 = MinLength { n: 3 };
    let r2 = MustContain {
        needle: "@".to_string(),
    };
    let rules: Vec<&dyn Validator> = vec![&r1, &r2];
    assert!(collect_errors(&rules, "a@b").is_empty());
}

#[test]
fn collect_errors_reports_all_failures_in_order() {
    // Three different concrete types, one slice. That's the trait
    // object payoff: pluggable rules, none of which know about each
    // other.
    let r1 = MinLength { n: 8 };
    let r2 = MustContain {
        needle: "@".to_string(),
    };
    let r3 = MustNotContain {
        forbidden: " ".to_string(),
    };
    let rules: Vec<&dyn Validator> = vec![&r1, &r2, &r3];
    // Input must trip all three rules:
    //   - shorter than 8 chars (fails MinLength)
    //   - no '@' anywhere   (fails MustContain)
    //   - contains a space  (fails MustNotContain)
    assert_eq!(
        collect_errors(&rules, "a b"),
        vec![
            "must be at least 8 characters".to_string(),
            "must contain '@'".to_string(),
            "must not contain ' '".to_string(),
        ]
    );
}

#[test]
fn collect_errors_with_no_rules_returns_empty() {
    let rules: Vec<&dyn Validator> = vec![];
    assert!(collect_errors(&rules, "anything").is_empty());
}
