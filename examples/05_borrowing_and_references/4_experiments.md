# Experiments: break it, then fix it

These tests pass as written.
Uncomment the lines in one experiment at a time to see what the compiler rejects.
Before you do, predict which line will cause an error and why.
Then run the tests and compare the error with your prediction.

1. In `experiment_use_after_move`, uncomment the assertion.
   Fix it without cloning the string or changing `take_ownership`.
2. In `experiment_two_mutable_borrows`, uncomment both lines.
   Fix the error by reordering the existing statements.
   Keep both references and both pushes; the final string should be `Ferris!!`.
3. In `experiment_mix_shared_and_mutable`, uncomment the mutation.
   Reorder the statements, keeping both the print through `shared` and the mutation.
   The print should show `Ferris`; the final string should be `Ferris - now with extra crab`.

For the borrowing experiments, don't clone the string, remove an operation, or add a nested scope.
Mark the last use of each reference and predict whether your new order will compile before running it.
Add an assertion for each final string.
Once the tests pass, explain why the borrows no longer conflict even though the reference bindings are still in scope.

Keep each fix before moving on.
To skip an experiment, comment out the lines you uncommented so its test passes again.
Even if you run only one test, all tests in the file still have to compile.
