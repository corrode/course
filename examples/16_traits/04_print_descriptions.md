# One Function, Many Types

Now write a caller that doesn't need to know which concrete type it is describing.
A trait bound lets a generic function require an interface without naming every type that might implement it.
The compiler checks that requirement at each call.

Define `print_descriptions`, including its generic parameter, trait bound, parameter type, and return type.
It must borrow a slice of any one type implementing `Describable` and return an owned `String`.
One call might receive labels; another might receive numbers.
It must also work with future implementations of the trait, without requiring extra traits such as `Clone` or `Display`.

Return each element's description in slice order, with one newline between neighboring descriptions and no extra separator at the end.
An empty slice returns an empty string.
An empty description is still an element: three descriptions `""`, `"middle"`, and `""` produce `"\nmiddle\n"`.
Do not print to standard output, consume the slice, or change its elements.

The tests supply small `Label` and `Number` implementations so this editor runs independently of the book/movie exercise.
Only your generic function is missing.
The initial compiler error about `print_descriptions` being undefined should disappear once you add it.

For now, every element in one call has the same concrete type.
We'll make a mixed collection later in this chapter.

## Useful from the Standard Library

- [`Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push) can collect owned descriptions from a loop.
- [`slice::join`](https://doc.rust-lang.org/std/primitive.slice.html#method.join) combines string elements with a separator, including empty elements.

You don't need iterator adapters for this exercise; those come in the next chapter.
