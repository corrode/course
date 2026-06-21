# `pub` on enums (and their variants)

Same idea as the previous step, but on an enum.
Mark the type public and the compiler accepts the call site.

One small bonus: enum variants inherit visibility from the enum itself.
Once the enum is `pub`, the variants come along for the ride.
Rust spares you the boilerplate, because an enum whose variants you can't name is useless to anyone outside the module.

Compile, read the error, fix it.
