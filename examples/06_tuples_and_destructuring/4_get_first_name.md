# Destructuring a tuple parameter

You can destructure a tuple right in the function parameter list, or
inside the body with a `let` binding. Either way, you pull out the
pieces by position.

Watch out for ownership: a tuple of `String`s is moved into the
function, while a tuple of integers is copied. The doc-comment below
has more on this.
