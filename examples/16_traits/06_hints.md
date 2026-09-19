# Hints

## display_temperature

`write!` takes a formatter followed by a format string and its arguments. It
already returns `fmt::Result`, so you can return that result directly. The
precision part of a floating-point format specifier controls how many digits
follow the decimal point.

## describable

Look back at the `Greet` example in the introduction for the shape of an
implementation block. The method signature must match the trait, including its
borrowed receiver. Use `format!` with the current value's fields rather than the
test's example text.

## print_descriptions

The type parameter belongs to the function, and its bound names the behavior the
body needs. Look back at the static-dispatch example, but use this exercise's
trait and return an owned result. A slice parameter borrows its elements, so the
bound need not require `Clone`. Collect the descriptions in a loop, then join
them with a separator so there is no trailing newline.

## logger

Implement only the required method for `PlainLogger`. The trait's other methods
already have bodies and will call your implementation of `log`. Return an owned
copy of the message without adding a prefix.

## logger_override

Add a method with the same signature as the trait's `error` method inside the
existing implementation block. Build the critical message and pass it through
`self.log`, so the tag is added in one place. There is no need to override
`warn`.

## validation_rules

Match the trait's method signature in both implementation blocks.
`str::contains` checks an entire substring, not whether any one character
appears. The two rules have opposite success conditions. Use the configured
field when constructing the failure message.

## mixed_shelf

The signature already says how the elements are borrowed. Calling `describe`
through each reference chooses that value's implementation. The string assembly
can use the same approach as the generic function.

## collect_errors

Inspect each rule's result and keep only its error message. A failure should not
stop the loop: the caller needs every message in rule order. Using `?` here
would express the wrong behavior.
