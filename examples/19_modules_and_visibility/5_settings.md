# `pub` on structs (and their methods)

You've now seen `pub` on a function and on an enum. Both followed
the same rule: `pub` the item, done. Structs are where that rule
breaks down.

Marking a `struct` `pub` makes the *type* visible from outside its
module. The fields and methods don't come along for the ride: each
one stays private until you opt it in individually. That's a
feature, not an inconvenience. It lets you expose the type while
keeping the implementation behind a curtain.

This step is broken in more than one place. Compile, read the
error, fix the smallest thing, compile again, and repeat until the
compiler runs out of complaints.
