# `pub` on enums (and their variants)

Enums follow the same rule: a `pub enum` exposes the type, but the
**variants** stay private until you opt them in. The shortcut is
`pub enum`, then list the variants. They inherit visibility from
the enum itself, *unlike* struct fields.

Wait, did I just contradict myself? Yes and no: `pub enum` makes the
variants public *by default* (because an enum without accessible
variants is useless), but you still need to write the `pub` on the
enum keyword itself. Try the change and read the compiler error if
you got it wrong.
