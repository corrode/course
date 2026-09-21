# Public Type, Private Fields

Callers outside `config` should be able to construct settings and read the port
through the existing methods. Keep the stored `port` field private.

The implementation is already there, but this step doesn't compile. Run it and
use the compiler's errors to find which declarations callers need to reach.
Make only the visibility changes needed; leave the function bodies as they are.

When you want something between fully public and fully private, `pub(crate)`
makes an item visible everywhere in your own crate while keeping it hidden from
outside users. Use it for helpers that several modules share but that aren't
part of your public API.

## Check the Boundary

Once the tests pass, temporarily replace `settings.get_port()` in the test with
`settings.port`. Read which declaration the compiler points to, then restore the
accessor call. Don't add `pub` to the field to silence this error.

Could you rename the stored field to `listen_port` without changing callers?
What if you narrowed it to `u16` instead? Consider `Settings::new(70000)` before
answering: hiding storage does not make changes to accepted values harmless.
