# Counting evens with `for` and `continue`

Now you have a slice of integers and want to know how many of them
are even. The natural shape is a `for` loop over the slice with a
counter that you bump on every match.

This is a good place to use `continue`: skip the odds early and the
"do work" branch ends up uncluttered.
