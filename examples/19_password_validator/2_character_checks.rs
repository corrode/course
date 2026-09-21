/// Checks for an ASCII uppercase letter without trimming or normalizing.
fn has_uppercase(password: &str) -> bool {
    todo!("Check whether the password contains an ASCII uppercase letter")
}

/// Checks for an ASCII lowercase letter without trimming or normalizing.
fn has_lowercase(password: &str) -> bool {
    todo!("Check whether the password contains an ASCII lowercase letter")
}

/// Checks for an ASCII digit without trimming or normalizing.
fn has_digit(password: &str) -> bool {
    todo!("Check whether the password contains an ASCII digit")
}

/// Checks for one of exactly `!@#$%^&*`.
fn has_special(password: &str) -> bool {
    todo!("Check whether the password contains one of !@#$%^&*")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uppercase_accepts_ascii_including_a_late_match() {
        for password in ["A", "Z", "é١!?lowercaseZ", " A "] {
            assert!(has_uppercase(password));
        }
    }

    #[test]
    fn uppercase_rejects_empty_other_classes_and_unicode() {
        for password in ["", "lowercase012!", "É", "é", "١", "?_-."] {
            assert!(!has_uppercase(password));
        }
    }

    #[test]
    fn lowercase_accepts_ascii_including_a_late_match() {
        for password in ["a", "z", "É١!?ABCz", " a "] {
            assert!(has_lowercase(password));
        }
    }

    #[test]
    fn lowercase_rejects_empty_other_classes_and_unicode() {
        for password in ["", "UPPERCASE012!", "É", "é", "١", "?_-."] {
            assert!(!has_lowercase(password));
        }
    }

    #[test]
    fn digit_accepts_ascii_including_a_late_match() {
        for password in ["0", "9", "Éé١!?Letters9", " 0 "] {
            assert!(has_digit(password));
        }
    }

    #[test]
    fn digit_rejects_empty_other_classes_and_unicode() {
        for password in ["", "Letters!", "É", "é", "١", "?_-."] {
            assert!(!has_digit(password));
        }
    }

    #[test]
    fn special_accepts_every_allowed_symbol_including_a_late_match() {
        for symbol in "!@#$%^&*".chars() {
            assert!(has_special(&symbol.to_string()));
            assert!(has_special(&format!("Éé١?Letters09{symbol}")));
        }
    }

    #[test]
    fn special_rejects_empty_other_classes_and_other_punctuation() {
        for password in [
            "",
            "Letters09",
            "É",
            "é",
            "١",
            "?_-.,:;/\\()[]{}+=~`|<>\"'",
            " \n\t",
        ] {
            assert!(!has_special(password));
        }
    }
}
