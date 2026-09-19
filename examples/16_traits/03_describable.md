# Implement a Shared Interface

A book and a movie store different information, but both can describe themselves.
The `Describable` trait below defines the interface; your job is to implement it for both types.
Write the implementation blocks and methods yourself rather than filling in prewritten bodies.

- A `Book` returns `"<title> by <author>"`, for example `"Dune by Herbert"`.
- A `Movie` returns `"<title> (<year>)"`, for example `"Arrival (2016)"`.

Use the fields of the value you receive, not fixed strings from the examples.
Descriptions must borrow the value without changing it, so callers can describe the same value again.
There is no generic function to write in this step.

The tests use `Describable::describe(&value)` to require an actual trait implementation.
Writing an unrelated method with the same name won't satisfy that contract.
Until you add the implementations, **Run** will report that the types do not implement `Describable`.
Those compiler errors are the starting point, not broken exercise setup.
