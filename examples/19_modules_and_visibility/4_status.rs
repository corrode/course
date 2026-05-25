// Module with an enum.
mod status {
    #[derive(Debug, PartialEq)]
    enum State {
        Running,
        Stopped,
    }
}

/// Returns the current state to anyone outside the `status` module.
fn get_status() -> status::State {
    status::State::Running
}

#[test]
fn test_get_status() {
    let state = get_status();
    assert_eq!(state, status::State::Running);
}
