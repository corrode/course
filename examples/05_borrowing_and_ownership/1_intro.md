# Borrowing and Ownership

In the moves chapter, `take_ownership` took a `String` and returned it. That
works, but passing ownership back and forth gets tedious fast when the function
only needs to *read* the value.

Instead, a better approach is to let the function "borrow" the value. 
A borrow lets a function use a value without taking ownership of it: `&value`
for a shared, read-only borrow, and `&mut value` for an exclusive, writable one.

```rust
fn length(s: &String) -> usize { s.len() }   // borrows, doesn't take

let s = String::from("rust");
let n = length(&s);    // lend s out for the call
println!("{s}");       // s still owns the data
```

The caller keeps ownership the whole time. The function gets temporary access
and gives it back when it returns.

## Shared or Exclusive Access

For the same value, the borrowing rule is:

> While references are in use, you can have either any number of shared `&`
> references, or one exclusive `&mut` reference. Never both at once.

A shared reference promises the data won't change while you're looking at it. A
mutable reference promises nobody else is looking while you write to it. For a
`String`, this prevents a write from reallocating the buffer while another
reference still points into it. Across threads, the same restriction also helps
prevent data races.

Borrowing can take a few tries to get used to, especially when you have to work
out where a borrow ends. When the compiler rejects your code, I suggest starting
with this rule: who needs to read the value, and who needs to change it?
