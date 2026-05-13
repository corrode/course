# Classifying a number

A three-way decision: return `"positive"` for any number greater
than zero, `"negative"` for anything below, and `"zero"` for the
exact match. The natural shape is `if`/`else if`/`else` returning
a `&'static str`.

`&'static str` (the same type you saw for error literals in the
result chapter, only earlier here) just means "a borrowed string
slice that lives for the whole program". String literals like
`"positive"` automatically have this type.

## Useful from the standard library

- The `if`/`else if`/`else` chain is one expression. Both branches
  have to return the same type. Each branch is a single string
  literal here, so no semicolons.
- The comparison operators `>`, `<`, `==` produce `bool`. No
  parentheses needed around the conditions.
