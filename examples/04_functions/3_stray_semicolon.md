# A stray semicolon

This function takes an `i32`, promises to return an `i32`, and multiplies the input by two.
Still, the compiler refuses to compile it.

Run the tests and read the error before you change anything.

The error points to the difference between an *expression*, which has a value, and a *statement*, which doesn't.
One character decides which one the final line is, and therefore what the function returns.
