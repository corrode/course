# A stray semicolon

This function takes an `i32`, declares an `i32` return type, multiplies by two.
Yet, the compiler refuses to compile. 

Run the tests, read the error, and fix it.

The lesson hiding behind that error is the difference between an *expression* (which has a value) and a *statement* (which doesn't).
One character decides which a line is, and that character decides what your function returns. 
