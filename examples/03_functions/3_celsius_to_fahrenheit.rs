/// Converts a Celsius temperature to Fahrenheit.
///
/// The formula is `(c * 9.0 / 5.0) + 32.0`. Both input and output
/// are `f64`.
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close_enough(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert!(close_enough(celsius_to_fahrenheit(0.0), 32.0));
        assert!(close_enough(celsius_to_fahrenheit(100.0), 212.0));
        assert!(close_enough(celsius_to_fahrenheit(-40.0), -40.0));
    }
}
