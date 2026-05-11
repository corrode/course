# Read config file

Same shape as `safe_divide`, but returning an owned `String`. Notice
you can mix `Ok(String::from("..."))` and `Err("...")` in the same
function: the success and error types are independent.
