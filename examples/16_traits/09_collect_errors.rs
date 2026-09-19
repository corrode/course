trait Validator {
    fn check(&self, input: &str) -> Result<(), String>;
}

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

/// Run every rule. Return failure messages in rule order, or an empty vector.
fn collect_errors(validators: &[&dyn Validator], input: &str) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failures_stay_in_order_with_a_pass_between_them() {
        let length = MinLength { n: 8 };
        let passing = MustContain { needle: "b".into() };
        let missing = MustContain { needle: "@".into() };
        let rules: [&dyn Validator; 3] = [&length, &passing, &missing];
        assert_eq!(
            collect_errors(&rules, "abc"),
            vec!["must be at least 8 characters", "must contain '@'"]
        );
    }

    #[test]
    fn all_pass_or_no_rules() {
        let length = MinLength { n: 3 };
        let required = MustContain { needle: "@".into() };
        assert!(collect_errors(&[&length, &required], "a@b").is_empty());
        assert!(collect_errors(&[], "anything").is_empty());
    }

    struct RejectInput;

    impl Validator for RejectInput {
        fn check(&self, input: &str) -> Result<(), String> {
            Err(format!("rejected '{input}'"))
        }
    }

    #[test]
    fn custom_rules_receive_the_input_and_keep_duplicate_messages() {
        let missing = MustContain { needle: "@".into() };
        let rules: [&dyn Validator; 3] = [&missing, &RejectInput, &RejectInput];
        assert_eq!(
            collect_errors(&rules, "abc"),
            vec!["must contain '@'", "rejected 'abc'", "rejected 'abc'"]
        );
    }
}
