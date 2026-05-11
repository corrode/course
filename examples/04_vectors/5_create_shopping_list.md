# Building a list from borrowed slices

The trickiest of the four: each input is a `&str`, but the output is
a `Vec<String>`. Each borrowed slice has to become an owned `String`
somewhere along the way. The `String::from` / `.to_string()` /
`.to_owned()` family all do this conversion.
