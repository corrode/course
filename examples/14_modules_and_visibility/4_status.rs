// Module with an enum.
// Make the enum variants comparable so the test below has something to assert on.
// (You'll still need to make `State` and its variants public to make `get_status`
// usable from outside the module.)
mod status {
    #[derive(Debug, PartialEq)]
    enum State {
        Running,
        Stopped,
    }
}

/// Returns the current state. To make this compile, make the `State`
/// enum public.
fn get_status() -> status::State {
    status::State::Running
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_status() {
        let state = get_status();
        // Once you make `State` public, this compiles and pins down the
        // actual return value, not just "it builds".
        assert_eq!(state, status::State::Running);
    }
}
