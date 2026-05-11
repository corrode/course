# Parse percentage

This is the hardest function in the chapter; the previous three were
warmups. More than one thing can go wrong, and they need different
error messages. Strip the optional `%` first, then `parse::<u8>()`
the rest, then bounds-check. Each step is its own potential `Err`.

Note: the error type here is `&'static str`, which means the message
has to be a string literal. If you find yourself wanting
`format!("{input} is out of range")` in an `Err`, you'd need to
change the return type to `Result<u8, String>`. Stick with literals
for this exercise.
