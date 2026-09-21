# Hints

## most_common_word

The return type owns its `String`. If the compiler reports a borrowed value,
check what your iterator yields and how long the map lives. You don't need to
keep the map after this function returns.

What should the comparison key measure: the word itself, or how often it
appeared? Also check what your chosen iterator method returns for an empty map
before adding a separate branch.
