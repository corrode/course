# Experiments: break it, then repair it

These tests start green. The commented lines let you opt into compiler errors, one experiment at a time.
Before uncommenting anything, predict whether the code will compile and which use might cause trouble. Then run the tests and compare the error with your prediction.

1. In `experiment_use_after_move`, enable the commented assertion. Repair the assertion without cloning the string or changing `take_ownership`.
2. In `experiment_two_mutable_borrows`, enable both commented lines. Repair the conflict by reordering the existing statements. Keep both references and both pushes; the final string should be `Ferris!!`.
3. In `experiment_mix_shared_and_mutable`, enable the mutation. Keep the print through `shared` and the mutation. The print should show `Ferris`; the final string should be `Ferris - now with extra crab`.

For the borrow repairs, don't clone the string, remove an operation, or add a nested scope. Mark the last use of each reference and predict whether your new order will compile before running it.
Add an assertion for each final string. Once a repair passes, try to explain why it works even though both reference bindings are still in scope.

Keep each successful repair before moving on. If you want to skip an experiment, comment its opt-in lines out again to restore the passing starting state.
A test filter cannot hide a compiler error: all tests in the file still have to compile.
The solution shows the repairs; the wrap-up explains where the borrows end.
