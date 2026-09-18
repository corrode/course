# Public Type, Private Fields

Making the type public is only the first layer.

`pub struct Settings` makes the *type* visible outside its module.
It does *not* make the fields public.
`port` stays private until you mark it `pub` on its own, even though the struct around it is public.

So a caller outside the module cannot write `settings.port`.
That line fails to compile.
A public accessor such as `get_port` lets callers read the value while leaving you free to change how you store it.

The same opt-in rule covers methods.
`new` and `get_port` are private until you `pub` each one, so this step is broken in more than one place.
Make the type, constructor, and accessor reachable, but keep `port` private.
Passing the test by making everything public misses the point: callers should
read settings through the API, not depend on their storage.

When you want something between fully public and fully private, `pub(crate)` makes an item visible everywhere in your own crate while keeping it hidden from outside users.
Use it for helpers that several modules share but that aren't part of your public API.

## Predict a Failure

Once the tests pass, temporarily replace `settings.get_port()` in the test with
`settings.port`. Will it compile? Predict the error, then run it and restore the
accessor call. Don't add `pub` to the field to silence this error.

Could you rename the stored field to `listen_port` without changing callers?
What if you narrowed it to `u16` instead? Consider `Settings::new(70000)` before
answering: hiding storage does not make changes to accepted values harmless.
