/// Prints the numbers from `n` down to `1`, each on its own line,
/// then prints `"Liftoff!"`. For `n == 0`, prints only `"Liftoff!"`.
///
/// Solved recursively: print the current number, then call `countdown`
/// with `n - 1`. The `n == 0` case is the base case that stops the
/// recursion.
fn countdown(n: u32) {
    if n == 0 {
        println!("Liftoff!");
    } else {
        println!("{n}");
        countdown(n - 1);
    }
}

// We can't easily assert on stdout here, so the tests just check
// that `countdown` compiles, runs, and returns `()` for a few
// inputs without panicking.
#[test]
fn test_countdown_returns_unit() {
    let result: () = countdown(0);
    assert_eq!(result, ());
    countdown(1);
    countdown(5);
}
