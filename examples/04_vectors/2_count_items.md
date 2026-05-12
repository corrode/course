# Counting items

The simplest possible operation on a vector is to ask it how many items it currently holds.
Notice the parameter is `&Vec<String>`: a borrow, so the caller keeps
ownership of the list.
