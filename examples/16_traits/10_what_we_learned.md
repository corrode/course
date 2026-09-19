# Wrapping Up Traits

You gave existing types a shared interface, then wrote callers that depend on
that interface rather than on concrete types. Separating those jobs let you test
each implementation before combining values in a collection.

## What We Learned

- A trait declares behavior; an implementation connects that behavior to a
  particular type. An inherent method with the same name does not make the type
  implement the trait.
- `Display` lets standard formatting code call your implementation. The standard
  library does not derive it because it cannot choose a human-readable
  representation for you.
- A generic trait bound lets one function work with different types across
  calls, while each call uses a single concrete type.
- A default method can call a required method supplied by the implementor.
  Override only the behavior that needs to differ.
- Borrowed trait objects let one collection refer to different concrete types
  through a common interface. They do not take ownership of the values.
- A caller decides how to handle each method's result. The validation collector
  keeps checking after errors so it can report every failure.

The optional [Smart Pointers](23_smart_pointers) chapter takes the next
ownership step: returning a collection that owns different command types rather
than borrowing them.
