# Hints

## calculate

1. The compiler complains about a private item with a path like
   `calculator::add`. Look at where `add` is declared.
2. Functions inside a module are private by default. Add `pub` in front of the
   `fn` keyword on the one the compiler is naming.

## settings

A type's visibility and the visibility of its fields and methods are separate.
Follow one compiler error at a time, then compile again. Which declaration is
named now?

Look at how the test reads the port. Does it need access to the stored field,
or only to a method? The caller's needs determine what you expose.
