# Methods that borrow `&self`

A method taking `&self` reads the struct's fields without modifying
or consuming it. The most common kind. Inside the method, `self`
behaves like any other reference, so you can read fields freely
and the caller keeps ownership.

`display_name` formats two fields into a new `String`. Use
`format!` rather than building the string by hand. It's the
idiomatic tool for this and reads exactly like the format you want.
