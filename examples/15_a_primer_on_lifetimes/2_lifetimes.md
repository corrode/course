# A First Look at Lifetimes

The borrow checker also tracks how long a reference stays valid. That's what a
*lifetime* describes.

A reference borrows a value, so it must not outlive that value. If it could,
you'd have a reference pointing at memory that's already been cleaned up, which
is a use-after-free bug.

Picture a function that tries to return a reference to its own local string:

```rust
fn dangling() -> &String {
    let s = String::from("temporary");
    &s   // ERROR: `s` is dropped at the end of this function
}
```

`s` is owned by `dangling`, so it's dropped the moment the function returns. A
reference to it would dangle, so the compiler rejects the code. When reading
this error, I'd start by asking who owns `s` and when it gets dropped. A borrow
has to stay valid for as long as you use it, and the compiler checks that span
(the reference's *lifetime*) against the owner's.

Lifetimes can have a name.
By convention, you name them `'a`, `'b`, and so on, though more descriptive names are allowed too.

```rust
fn dangling<'a>() -> &'a String {
    let s = String::from("temporary");
    &s   // Still an error: naming a lifetime doesn't keep `s` alive
}
```

In this example, `<'a>` declares a lifetime parameter, and `&'a String` uses it
for the returned reference. 

This is the same attempt to return a reference to a local value, and it fails
for the same ownership reason. Strictly speaking, the first signature also
has a missing lifetime: there is no input reference whose lifetime the compiler
can use for the return value. Writing `'a` addresses that missing annotation,
but it doesn't extend the life of `s`. To return this string, return an owned
`String` instead of a `&String` reference. 

Most of the time the compiler works lifetimes out on its own and you never write
one. You'll usually need annotations when a struct holds a reference or a
function returns one of several borrowed inputs. The syntax can take some
getting used to, but keep the same question in mind: which owner does this
reference depend on, and will that owner still be alive?

> [!TIP]
>
> When the compiler reports a lifetime error, trace the borrowed value, its owner,
> and the last use of the reference.
