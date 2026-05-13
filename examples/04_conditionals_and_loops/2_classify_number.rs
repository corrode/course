/// Classifies a number as `"positive"`, `"negative"`, or `"zero"`.
///
/// One `if`/`else if`/`else` chain, used as the function's return
/// expression.
fn classify_number(n: i32) -> &'static str {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_number() {
        assert_eq!(classify_number(7), "positive");
        assert_eq!(classify_number(-3), "negative");
        assert_eq!(classify_number(0), "zero");
        assert_eq!(classify_number(i32::MAX), "positive");
        assert_eq!(classify_number(i32::MIN), "negative");
    }
}
