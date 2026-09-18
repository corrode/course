# A first look at lifetimes

The borrow checker also tracks how long a reference stays valid.
That's what a *lifetime* describes.

A reference borrows a value, so it must not outlive that value.
If it could, you'd have a reference pointing at memory that's already been cleaned up, which is a use-after-free bug.

Picture a function that tries to return a reference to its own local string:

```rust
fn dangling() -> &String {
    let s = String::from("temporary");
    &s   // ERROR: `s` is dropped at the end of this function
}
```

`s` is owned by `dangling`, so it's dropped the moment the function returns.
A reference to it would dangle, so the compiler rejects the code.
When reading this error, I'd start by asking who owns `s` and when it gets dropped.
A borrow has to stay valid for as long as you use it, and the compiler checks that span (the reference's *lifetime*) against the owner's.

Most of the time the compiler works lifetimes out on its own and you never write one.
You'll usually need annotations when a struct holds a reference or a function returns one of several borrowed inputs.
The syntax can take some getting used to, but keep the same question in mind: which owner does this reference depend on, and will that owner still be alive?

You don't need the syntax yet.
For now, recognize this kind of error and connect it to the same safety rule you've already been using.

When the compiler reports a lifetime error, trace the borrowed value, its owner, and the last use of the reference.
