/// A text transformation that borrows its command and returns an owned string.
trait Command {
    fn run(&self, input: &str) -> String;
}

/// Uppercase the input.
struct Uppercase;

impl Command for Uppercase {
    fn run(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

/// Reverse the input by Unicode scalar value.
struct Reverse;

impl Command for Reverse {
    fn run(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
}

/// Append an owned suffix.
struct Append {
    suffix: String,
}

impl Command for Append {
    fn run(&self, input: &str) -> String {
        format!("{input}{}", self.suffix)
    }
}

/// Return exactly two owned commands: Uppercase, then Append with the supplied suffix.
fn make_pipeline(suffix: String) -> Vec<Box<dyn Command>> {
    vec![Box::new(Uppercase), Box::new(Append { suffix })]
}

/// Pass input through every command in slice order and return the final output.
/// An empty pipeline returns the input unchanged.
/// Borrow the pipeline so it can be reused, and support any Command implementation.
fn apply_pipeline(commands: &[Box<dyn Command>], input: &str) -> String {
    let mut current = input.to_string();
    for command in commands {
        current = command.run(&current);
    }
    current
}

#[test]
fn factory_returns_uppercase_then_append() {
    let pipeline = make_pipeline(String::from("x"));
    assert_eq!(pipeline.len(), 2);

    // Inspect each stage independently of apply_pipeline.
    assert_eq!(pipeline[0].run("hi"), "HI");
    assert_eq!(pipeline[1].run("hi"), "hix");
    assert_eq!(pipeline[1].run(&pipeline[0].run("hi")), "HIx");
}

#[test]
fn factory_owns_the_supplied_suffix() {
    let pipeline = {
        let suffix = String::from(" fin");
        make_pipeline(suffix)
    };
    assert_eq!(pipeline.len(), 2);
    assert_eq!(pipeline[1].run("one"), "one fin");
    assert_eq!(pipeline[1].run("two"), "two fin");

    let empty_suffix = make_pipeline(String::new());
    assert_eq!(empty_suffix.len(), 2);
    assert_eq!(empty_suffix[1].run("hi"), "hi");
}

#[test]
fn empty_pipeline_returns_input_unchanged() {
    let pipeline: Vec<Box<dyn Command>> = Vec::new();
    assert_eq!(apply_pipeline(&pipeline, "hello"), "hello");
    assert_eq!(apply_pipeline(&pipeline, ""), "");
}

#[test]
fn single_command_uppercase() {
    let pipeline: Vec<Box<dyn Command>> = vec![Box::new(Uppercase)];
    assert_eq!(apply_pipeline(&pipeline, "hello"), "HELLO");
}

#[test]
fn order_matters() {
    let append_then_uppercase: Vec<Box<dyn Command>> = vec![
        Box::new(Append {
            suffix: "x".to_string(),
        }),
        Box::new(Uppercase),
    ];
    let uppercase_then_append: Vec<Box<dyn Command>> = vec![
        Box::new(Uppercase),
        Box::new(Append {
            suffix: "x".to_string(),
        }),
    ];
    assert_eq!(apply_pipeline(&append_then_uppercase, "hi"), "HIX");
    assert_eq!(apply_pipeline(&uppercase_then_append, "hi"), "HIx");
}

#[test]
fn mixed_pipeline_with_append() {
    let pipeline: Vec<Box<dyn Command>> = vec![
        Box::new(Append {
            suffix: "!".to_string(),
        }),
        Box::new(Uppercase),
        Box::new(Reverse),
    ];
    assert_eq!(apply_pipeline(&pipeline, "hi"), "!IH");
}

#[test]
fn borrowed_pipeline_supports_custom_commands_and_reuse() {
    // This implementation exists only in the test, outside the supplied command set.
    struct Bracket;

    impl Command for Bracket {
        fn run(&self, input: &str) -> String {
            format!("[{input}]")
        }
    }

    let pipeline: Vec<Box<dyn Command>> = vec![
        Box::new(Bracket),
        Box::new(Append {
            suffix: "x".to_string(),
        }),
    ];
    assert_eq!(apply_pipeline(&pipeline, "hi"), "[hi]x");
    assert_eq!(apply_pipeline(&pipeline, "bye"), "[bye]x");
    assert_eq!(apply_pipeline(&pipeline, "hi"), "[hi]x");
}
