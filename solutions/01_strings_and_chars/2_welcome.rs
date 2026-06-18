/// Your first exercise, how exciting!
fn format_welcome_message(name: &str) -> String {
    format!("Welcome, {name}!")
}

// Tests live right next to the code they exercise. Don't worry about the syntax yet.

#[test]
fn test_format_welcome_message() {
    assert_eq!(format_welcome_message("Alice"), "Welcome, Alice!");
    assert_eq!(format_welcome_message("Bob"), "Welcome, Bob!");
}
