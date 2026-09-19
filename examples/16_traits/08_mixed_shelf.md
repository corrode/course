# Borrowing a Mixed Shelf

The generic description function works for many types, but each call still
chooses one concrete element type. A slice of books cannot also hold a movie.
Now we want both on the same shelf, without taking ownership of either.

A borrowed trait object, `&dyn Describable`, refers to a value through its
shared interface. It carries access to both the value and the methods for its
concrete type. Different implementors can therefore appear in one slice of
trait-object references. This uses dynamic dispatch rather than selecting a
concrete implementation through a generic type parameter.

The book and movie implementations are supplied in this editor. Implement
`print_mixed_descriptions` to return their descriptions in slice order,
separated by newlines. An empty shelf returns an empty string. Borrow the
existing values; don't clone them or move them into boxes. The function must
work with other implementations of `Describable` too.

Compare its supplied parameter type with the generic signature you wrote
earlier. Why does putting each trait object behind a reference make a mixed
slice possible? The owning form, `Box<dyn Trait>`, is a separate topic in the
optional [Smart Pointers](23_smart_pointers) chapter.
