/// A single step in a text-transformation pipeline.
///
/// `dyn Command` is unsized (different implementors have different
/// sizes), so values of type `dyn Command` always live behind some
/// kind of pointer. `Box<dyn Command>` is the owned form.
trait Command {
    fn run(&self, input: &str) -> String;
}

/// Upper-case the input.
struct Uppercase;

impl Command for Uppercase {
    fn run(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

/// Reverse the input (by Unicode scalar value).
struct Reverse;

impl Command for Reverse {
    fn run(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
}

/// Append a fixed suffix.
struct Append {
    suffix: String,
}

impl Command for Append {
    fn run(&self, input: &str) -> String {
        format!("{input}{}", self.suffix)
    }
}

/// Thread `input` through every command in order, feeding each
/// command's output into the next command's input. Returns the
/// final string.
///
/// An empty pipeline returns `input` unchanged.
///
/// The slice element type is `Box<dyn Command>`, so the caller can
/// mix `Uppercase`, `Reverse`, `Append` (and any future implementor)
/// freely in a single pipeline.
fn apply_pipeline(commands: &[Box<dyn Command>], input: &str) -> String {
    todo!()
}

#[test]
fn empty_pipeline_returns_input_unchanged() {
    let pipeline: Vec<Box<dyn Command>> = Vec::new();
    assert_eq!(apply_pipeline(&pipeline, "hello"), "hello");
}

#[test]
fn single_command_uppercase() {
    let pipeline: Vec<Box<dyn Command>> = vec![Box::new(Uppercase)];
    assert_eq!(apply_pipeline(&pipeline, "hello"), "HELLO");
}

#[test]
fn two_commands_in_order() {
    // hello -> HELLO -> OLLEH
    let pipeline: Vec<Box<dyn Command>> = vec![Box::new(Uppercase), Box::new(Reverse)];
    assert_eq!(apply_pipeline(&pipeline, "hello"), "OLLEH");
}

#[test]
fn order_matters() {
    // hello -> olleh -> OLLEH
    let pipeline: Vec<Box<dyn Command>> = vec![Box::new(Reverse), Box::new(Uppercase)];
    assert_eq!(apply_pipeline(&pipeline, "hello"), "OLLEH");
    // Same commands, opposite order, same result here only because
    // both ops are case- and order-symmetric on ASCII. The next test
    // exercises a non-symmetric case.
}

#[test]
fn mixed_pipeline_with_append() {
    // hi -> hi! -> HI! -> !IH
    let pipeline: Vec<Box<dyn Command>> = vec![
        Box::new(Append {
            suffix: "!".to_string(),
        }),
        Box::new(Uppercase),
        Box::new(Reverse),
    ];
    assert_eq!(apply_pipeline(&pipeline, "hi"), "!IH");
}
