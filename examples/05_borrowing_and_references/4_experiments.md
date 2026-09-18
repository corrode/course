# Experiments: get the errors on purpose

Passing the previous tests verifies the basic borrowing syntax.
Now you'll deliberately break the rules and use the compiler messages to work out what went wrong.

Each test below contains commented-out code.
Follow its comment to uncomment one line (or the pair of lines in the second test), run the tests, and read the error.
Comment the code out again before moving on.

The three errors illustrate these ownership and borrowing restrictions:

1. You can't use a value after you've moved it.
2. You can't have two mutable references to the same value at once.
3. You can't have a mutable reference while a shared reference is still in use.

Re-read each compiler message until you can explain in one sentence *why* the compiler is complaining.
Once you can do that, you can change the code for a reason instead of guessing.
