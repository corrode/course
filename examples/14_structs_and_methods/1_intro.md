# Structs and Methods

In Rust, a method's signature tells you whether it borrows a value, changes it,
or takes ownership of it. You group related fields in a `struct` and attach
methods with an `impl` block.

```rust
struct Playlist {
    tracks: Vec<String>,
}

impl Playlist {
    /// No receiver is needed to create an empty playlist.
    fn new() -> Self {
        Self { tracks: Vec::new() }
    }

    /// A shared borrow is enough to inspect the playlist.
    fn len(&self) -> usize {
        self.tracks.len()
    }

    /// Adding a track changes the playlist, so it needs a mutable borrow.
    fn add(&mut self, track: String) {
        self.tracks.push(track);
    }
}

let mut playlist = Playlist::new();
playlist.add("Blue in Green".to_string());
assert_eq!(playlist.len(), 1);
```

The form of `self` tells you what access the method receives:

- `&self` takes a shared borrow. You'll use this form for methods that only need
  to inspect fields.
- `&mut self` takes a mutable borrow, allowing the method to modify the struct
  in place. The caller therefore needs a mutable binding.
- `self` (no reference) consumes the struct, taking ownership. Choose this form
  when the method returns a transformed value and the original shouldn't be
  reused.

Field access uses dot notation (`playlist.tracks`). Inside `impl` you write
`self.field` for the same thing.

`Self` (capital S) is shorthand for "the type I'm `impl`ing". `Playlist` and
`Self` are interchangeable inside `impl Playlist`.

## A Note on Ranges: `0..5`

The `record_login` test calls the method five times in a loop:

```rust
for _ in 0..5 {
    user.record_login();
}
```

`0..5` is a *range expression*. It produces the values `0, 1, 2, 3, 4`
(end-exclusive). `0..=5` is the inclusive variant if you want `5` too. The loop
variable is `_` here because the body doesn't need it; we just want "do this
thing five times." Ranges are useful as iterators but also work as slice indices
(`v[0..3]`).

> [!TIP]
> From this chapter onward the files get longer, and the in-browser editor
> starts feeling cramped. The **Open in Web Editor** button above each editor
> opens this file on [github.dev](https://github.dev), a full browser-based VS
> Code with proper find-in-file, multi-cursor, and the rest of the keyboard
> shortcuts you'd expect. Clone the repo locally if you want `rust-analyzer` and
> on-save formatting.


