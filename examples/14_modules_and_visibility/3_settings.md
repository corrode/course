# `pub` on structs (and their methods)

A `pub struct` makes the *type* visible from outside the module, but
its **fields** and **methods** stay private until you also `pub`
them individually. That's a feature, not an inconvenience: it lets
you expose the type while keeping the implementation private.

This step is also deliberately broken in three places. Compile, read
the error, fix the smallest thing, and repeat.
