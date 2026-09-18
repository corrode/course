# A Mixed Shelf

Your shelf now contains a book and a movie. Would you keep `T: Describable`,
use borrowed trait objects, or introduce an enum? Choose before opening the code.
Assume other code may add more types implementing `Describable`.

The earlier `Book` and `Movie` implementations are supplied here.
Implement `print_mixed_descriptions`; leave your earlier generic function unchanged.
The tests borrow existing values: the collection must not take ownership or clone
them. Owning a mixed collection is a separate decision; you don't need `Box` here.
Explain why `&[T]` alone cannot accept both concrete types in one call.
