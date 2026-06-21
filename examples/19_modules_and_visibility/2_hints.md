# Hints

## calculate

1. The compiler complains about a private item with a path like `calculator::add`.
   Look at where `add` is declared.
2. Functions inside a module are private by default.
   Add `pub` in front of the `fn` keyword on the one the compiler is naming.

## settings

1. The first error is about the `Settings` *type* being private.
   `pub` it.
   Compile again.
   The next error is about `Settings::new` being private.
   `pub` it.
   Compile again.
   The next error is about `get_port` being private.
   You see where this is going.
2. `pub struct Settings` does not make the fields or methods public.
   Each item gets its own `pub`.
3. Leave `port` itself private.
   The test reaches it through `get_port`, which is the point: the field stays private even though the struct is public.
