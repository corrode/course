//! # Integer Operations
//!
//! Numbers are everywhere in programming! From calculating your grocery bill
//! to powering the algorithms that sent humans to the moon (thanks, Margaret Hamilton!),
//! integer arithmetic is fundamental to computer science.
//!
//! Fun fact: The first computer bug was literally a bug - Grace Hopper found
//! a moth stuck in a Harvard Mark II computer in 1947. But today, we'll focus
//! on avoiding logical bugs in our number handling code.
//!
//! You've got this! Let's crunch some numbers! 🔢

/// Convert a number to a string
fn number_to_string(number: u32) -> String {
    todo!()
}

/// Calculates the total price including tax.
/// Tax rate is given as a percentage (e.g., 8.5 for 8.5% tax).
fn calculate_total_with_tax(price: u32, tax_rate: f64) -> u32 {
    todo!()
}

/// Parses a string into a positive integer.
/// Returns the number if valid, 0 if invalid.
fn parse_positive_integer(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_calculation() {
        assert_eq!(calculate_total_with_tax(100, 8.5), 108);
        assert_eq!(calculate_total_with_tax(50, 10.0), 55);
    }

    #[test]
    fn test_integer_parsing() {
        assert_eq!(parse_positive_integer("123"), 123);
        assert_eq!(parse_positive_integer("0"), 0);
        assert_eq!(parse_positive_integer("invalid"), 0);
        assert_eq!(parse_positive_integer("-5"), 0);
    }

    #[test]
    fn test_number_formatting() {
        assert_eq!(number_to_string(1234), "1234");
        assert_eq!(number_to_string(0), "0");
    }
}
