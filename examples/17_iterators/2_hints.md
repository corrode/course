# Hints

## `sum`

An array isn't an iterator yet. Once you have an iterator, which consumer
produces a number rather than a collection? The function's return type can
supply the numeric type.

## `map`

Each input should produce exactly one output. With `into_iter`, your closure
receives an owned `String`. The transformation returns another `String`; you
don't need a mutable reference to the original.

## `filter`

The predicate decides whether to keep an item; it doesn't return the item
itself. With an iterator of `&str`, that predicate receives `&&str`. String
method calls auto-deref through the extra reference.

## `filter_to_string`

There are two separate jobs: deciding which paths belong in the result and
making the result independent of the input. Filtering alone doesn't change the
item type. Where in your pipeline do borrowed strings become owned ones?

## `fallible_sum`

What type does parsing each token produce? Compare that with the items a
fallible `sum` can consume. If you unwrap each parse result first, what happens
to the error you were supposed to return?

## `lazy_consumption`

1. Creating a `map` adapter doesn't call its closure. Something must ask for an
   item.
2. `filter` may request several inputs before it can yield one output.
3. `by_ref` borrows the existing iterator; consuming that borrow advances the
   original too.
