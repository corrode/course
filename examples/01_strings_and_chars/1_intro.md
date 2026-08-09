# Strings, &str, and chars

*An `i32` walks up to a `String` and asks for its number. The `String` replies: "Sorry, you're not my type."*

Many other languages have a mushy concept of a "string" that can be anything from a null-terminated byte array to a UTF-16 buffer to something else entirely (I'm looking at you, [Perl](https://www.perltutorial.org/perl-string/)).
Rust splits "string" across distinct types:

- `char` is one Unicode scalar value (always 4 bytes).
- `&str` is a borrowed view into UTF-8 text.
  Cheap to pass around.
- `String` is an owned, growable UTF-8 buffer.
  You own the memory.

Before we go further, let's take a moment to briefly introduce two words that I will use a lot going forward:

- **Owned** means *this value is mine; when I go out of scope, the memory behind it is freed*.
  In C++ terms, it's the object held by `std::unique_ptr`; in Python or Java terms, it's the role of the variable that decides when the object can be collected.
  In Rust every heap value has exactly one owner at a time.
- **Borrowed** means *I'm looking at someone else's value without taking it over*.
  It's the equivalent of passing a `const T&` in C++, or handing out a read-only pointer in C.
  Borrows are written with an `&` (or `&mut` if you also want to mutate).
  The borrow has to end before the owner is dropped, and the compiler enforces that for you, ruling out use-after-free and dangling pointers.

This *ownership model* is why Rust has two string types in the first place: it tracks who owns each piece of data.
The brief version is that every value has one owner, the value is dropped (deleted) when that owner goes out of scope, and you can borrow a value without owning it.
The useful mental model is "one owner, many borrows."

The split between `&str` and `String` is what makes Rust strings both **fast and safe**.
A function that just *reads* text takes `&str` and Rust avoids any unnecessary copies or allocations.
A function that *produces* new text returns `String` and returns ownership to the caller, who can then decide what to do with it.

You'll see this pattern again and again:

```rust
fn shout(text: &str) -> String {
    text.to_uppercase()
}

let s = String::from("hello");
let louder = shout(&s); // &String coerces to &str
```

- **`&str`** ("string slice", pronounced *stir*) is a **borrowed view into text** that lives somewhere else.
  Taking `name: &str` means "I'll just need to read this string; I'm not taking ownership of it."
- **`String`** is *owned* and heap-allocated.
  Returning `-> String` means the caller gets a fresh, owned value back.

There's one common gotcha: If you call `.len()` on a string, it returns the number of *bytes*, not the number of characters in that string. 
Rust uses UTF-8 for strings, which means a single visible character can take more than one byte.
So if you rather need the number of "human-readable" characters in a string, use `s.chars().count()` instead.

`.chars()`  lets you walk through the `char` values in a string, one at a time.
The returned value is an *iterator* over those characters.
`Iterator` provides methods such as `.next()`, `.count()`, and `.any(...)` for consuming or inspecting its values.
We'll get to iterators in more detail later.

## Building a `String` with `format!`

A convenient way to assemble a new `String` is the `format!` macro.
It works like `println!`, except instead of printing, it returns the formatted text:

```rust
let name = "Alice";
let greeting: String = format!("Hello, {name}!");
```

In the format string, `{name}` is a **captured identifier**.
Rust pulls the variable from the surrounding scope.
Sometimes you still see `format!("Hello, {}!", name)` instead, which is the pre-2021 version, but both forms still work.
The exclamation mark (`!`) means `format!` is a macro rather than a regular function call.

## Consuming an iterator 

The simplest way to consume an iterator is a `for` loop:

```rust
for c in "hello".chars() {
    println!("{c}");
}
```

You can read it as "for each `c` produced on the right, run the body once."
In this case, for each character in the string `"hello"`, do something with it.
The loop variable is a fresh binding scoped to each iteration.
Ranges, arrays, and collections can also go on the right-hand side because each can produce an iterator.

## Where to look things up

- [`std::fmt`](https://doc.rust-lang.org/std/fmt/) contains everything the formatting macros can do (padding, precision, hex, debug output…).
- [`str`](https://doc.rust-lang.org/std/primitive.str.html): the inventory of operations available on any `&str`.
