# Public type, private fields

Making the type public is only the first layer.

`pub struct Settings` makes the *type* visible outside its module.
It does *not* make the fields public.
`port` stays private until you mark it `pub` on its own, even though the struct around it is public.

So a caller outside the module cannot write `settings.port`.
That line fails to compile.
A public accessor such as `get_port` gives callers a stable path to the value while the field remains free to change.

The same opt-in rule covers methods.
`new` and `get_port` are private until you `pub` each one, so this step is broken in more than one place.
Compile, read the error, `pub` the item it names, and repeat until the compiler runs out of complaints.

When you want something between fully public and fully private, `pub(crate)` makes an item visible everywhere in your own crate while keeping it hidden from outside users.
Use it for helpers that several modules share but that aren't part of your public API.
