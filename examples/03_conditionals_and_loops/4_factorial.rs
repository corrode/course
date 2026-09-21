/// Returns `n!` (n factorial). By convention, `factorial(0) == 1`.
fn factorial(n: u32) -> u32 {
    todo!("Return n factorial using a for loop")
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(10), 3_628_800);
}
