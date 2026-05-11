# Counting items

Rust's `Vec<T>` is a growable, heap-allocated array. The simplest
possible operation is to ask it how many items it currently holds.

Notice the parameter is `&Vec<String>`: a borrow, so the caller keeps
ownership of the list.
