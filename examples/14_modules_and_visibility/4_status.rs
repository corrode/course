//! # `pub` on enums (and their variants)
//!
//! Enums follow the same rule: a `pub enum` exposes the type, but the
//! **variants** stay private until you opt them in. The shortcut is
//! `pub enum`, then list the variants — they inherit visibility from
//! the enum itself, *unlike* struct fields.
//!
//! Wait, did I just contradict myself? Yes and no: `pub enum` makes the
//! variants public *by default* (because an enum without accessible
//! variants is useless), but you still need to write the `pub` on the
//! enum keyword itself. Try the change and read the compiler error if
//! you got it wrong.

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
