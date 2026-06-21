use std::fmt;

#[derive(Debug, PartialEq)]
struct Temperature {
    celsius: f64,
}

/// Formats the temperature as `"<value>°C"` with one decimal place.
/// Examples:
///   - `Temperature { celsius: 21.5 }`  → `"21.5°C"`
///   - `Temperature { celsius: -3.0 }`  → `"-3.0°C"`
///   - `Temperature { celsius: 100.0 }` → `"100.0°C"`
///
/// Hint: `write!(f, "{:.1}°C", self.celsius)` does the whole job.
/// The `:.1` is the same format specifier you'd use in `println!`.
impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.celsius)
    }
}

#[test]
fn test_display_room_temperature() {
    let t = Temperature { celsius: 21.5 };
    assert_eq!(format!("{t}"), "21.5°C");
}

#[test]
fn test_display_negative() {
    let t = Temperature { celsius: -3.0 };
    assert_eq!(format!("{t}"), "-3.0°C");
}

#[test]
fn test_display_boiling() {
    let t = Temperature { celsius: 100.0 };
    assert_eq!(format!("{t}"), "100.0°C");
}

#[test]
fn test_display_rounds_to_one_decimal() {
    // `{:.1}` rounds, it doesn't truncate.
    let t = Temperature { celsius: 18.27 };
    assert_eq!(format!("{t}"), "18.3°C");
}
