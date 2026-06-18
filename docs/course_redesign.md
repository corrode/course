# Course Redesign Blueprint

This document is the definitive plan for rebuilding the course from the ground up.
It incorporates findings from chapter-by-chapter audits, feedback from live workshop
cohorts, and deliberate pedagogical thinking about what makes a Rust beginner course
genuinely better than existing resources.

No backwards-compatibility is required. Rebuild cleanly.

---

## Why this redesign

Workshop participants on days 1 and 2 reported frustration:

- Too much theory before getting their hands dirty
- Early exercises felt trivial, then ownership hit like a wall
- Couldn't answer "why am I learning Rust specifically?"
- Progress felt invisible

The root cause is **structural**: the course is organized by language feature
(integers → strings → enums → …), which delays both motivation and the hard concepts.
Learners spend 10 chapters building false confidence, then hit ownership as a surprise
cliff. Chapters 0–8 are too easy not because they're well-scaffolded, but because the
hard parts have been postponed.

This redesign fixes the root cause, not the symptoms.

---

## Pedagogical principles

These principles govern every decision below. When in doubt, come back to them.

1. **Motivation before mechanics.** Every chapter that introduces a Rust-specific
   concept opens with the industry problem it solves — not the API. The learner should
   understand *why this exists* before being asked to use it.

2. **Spiral ownership.** Ownership is not a chapter — it is a thread. Concepts are
   introduced in small, motivated doses from chapter 1 onward, so the dedicated
   ownership chapter is a consolidation, not a revelation.

3. **Exercises from line one.** There are no read-only chapters. Every chapter has
   at least one `todo!()` to fill in. The edit → run → green tick loop is the
   primary learning mechanism.

4. **`?` belongs with `Result`.** Learners should never write verbose `match`/`unwrap`
   chains for more than one chapter. The `?` operator is taught in the same chapter as
   `Result`, immediately after the ugly version, so every exercise from that point
   forward uses idiomatic error handling.

5. **Closures before combinators.** `Option::map`, `Iterator::filter`, and every other
   higher-order method require closures. Closures get their own chapter before `Option`
   so that those methods are immediately usable without a sidebar detour.

6. **Difficulty transparency.** Every chapter and every exercise carries a difficulty
   label. Learners should never wonder "is this supposed to be hard?" The two main
   cliffs (ownership consolidation and the CSV parser) are announced in advance.

7. **The four pillars are the spine.** The four things that make Rust different from
   every other language a working developer has used — no null, no unchecked errors,
   memory safety without a GC, fearless concurrency — are introduced as the problems
   they solve, not the features they are. These four chapters are the emotional core of
   the course.

8. **Every promise is delivered.** If a chapter title is "Fearless Concurrency", the
   chapter must show a data race that the borrow checker refuses to compile, and show
   how to fix it. Promises made in the "Why Rust?" opening must have a corresponding
   chapter that pays them off.

---

## Chapter map

| # | Title | Theme | Difficulty | Spiral moment |
|---|---|---|---|---|
| — | Why Rust? | Motivation | Trivial | — |
| 0 | When Types Mean What They Say | Numbers, no implicit coercions | Easy | `Copy` types introduced |
| 1 | Owned or Borrowed | Strings, UTF-8 | Easy | **Spiral 1**: `&str` borrows vs `String` owns; the 3 rules, briefly |
| 2 | Control Flow | Conditionals, loops | Easy | — |
| 3 | Functions as Contracts | Functions, parameter types | Easy | **Spiral 2**: passing `String` moves it; passing `&str` borrows it |
| 4 | Functions as Values | Closures | Moderate | Capturing from environment; `move` closures |
| 5 | The Billion-Dollar Mistake, Fixed | `Option<T>` | Moderate | — |
| 6 | Errors You Cannot Ignore | `Result<T,E>` + `?` | Moderate | — |
| 7 | Making Impossible States Unrepresentable | Enums, pattern matching | Moderate | — |
| 8 | Structured Data Without the Baggage | Structs, methods | Moderate | **Spiral 3**: `&self` / `&mut self` as parameter ownership |
| 9 | Memory Safety Without a Garbage Collector | Ownership, borrowing | **Spike** | Spiral payoff: the three rules, now written down |
| 10 | Shared Behavior Without Inheritance | Traits | Moderate | — |
| 11 | Zero-Cost Abstractions | Iterators | Hard | — |
| 12 | Heap Memory, Under Control | Smart pointers | Moderate | — |
| 13 | Fearless Concurrency | Threads, `Arc`, `Mutex` | Hard | — |
| 14 | Visibility at Scale | Modules, visibility | Easy | — |
| 15 | Project: Word Frequencies | Synthesis | Moderate | — |
| 16 | Project: Environment Parser | Synthesis | Hard | — |
| 17 | Project: CSV Parser | Synthesis | **Spike** | — |
| — | Appendix: How This Course Is Designed | Meta | — | — |

**Total: 18 chapters + preamble + appendix.** The quiz (currently ch. 22) can remain
as a lightweight checkpoint after chapter 9 (the ownership spike) rather than at the
very end.

---

## Pre-course: "Why Rust?"

**Format:** Read-only. No exercises. Target: 5 minutes.

**Goal:** Answer "why Rust?" before the learner writes a single line. By the end, they
should *want* the compiler to be strict.

**Structure:**

1. Open with the four systemic problems that have plagued the industry for decades,
   each with a one-sentence war story:
   - **The billion-dollar mistake**: Tony Hoare's 1965 null reference. Every Java
     `NullPointerException`, every Python `AttributeError: 'NoneType'`, every
     JavaScript `Cannot read properties of undefined`. One invention, decades of
     production outages.
   - **Silent failures**: A function returns an error code. The caller ignores it.
     The system silently corrupts data. C, Go, and Java all let this compile.
   - **Memory unsafety**: Use-after-free. Buffer overflow. Double-free. The majority
     of CVEs in Chrome, Windows, and the Linux kernel are memory safety bugs in C and
     C++ code written by experts.
   - **Data races**: Two threads write to the same memory. No compiler warning. No
     runtime error on most runs. A crash in production at 3am.

2. State Rust's answer to each: `Option<T>`, `Result<T,E>` with `?`, the ownership
   system, the borrow checker.

3. Show ONE compiler error — a good one, with the helpful message — and frame it as:
   *"This is not the compiler being pedantic. This is the compiler catching a bug
   before you ship it."*

4. Promise: *"By the end of this course, all four of those problems will be tools you
   know how to use, not rules you've been told to memorize."*

**Tone:** Personal and direct. First person. This is the one place where you can say
"I've spent years helping teams adopt Rust, and these are the four things that changed
how they think about software."

---

## Chapter specs

### Chapter 0 · When Types Mean What They Say
*Difficulty: Easy*

**Opening hook:** In C, `int i = 2.9` silently truncates to `2`. In JavaScript,
`0.1 + 0.2` is `0.30000000000000004`. Rust refuses both: every numeric conversion is
written out explicitly, so the code says exactly what it does.

**Concepts:** Integer types (`i32`, `u32`, `usize`, `f64`), `as` casts, `f64::round()`,
`str::parse()`, `to_string()`. Introduce `Copy` types with one sentence: *"Integers are
cheap to copy; Rust duplicates them automatically. Not all types are this lucky —
you'll see why in chapter 1."*

**Exercises:**
- `number_to_string(n: u32) -> String`
- `calculate_damage(base: u32, multiplier: f64) -> u32` — explicit cast, truncation
  semantics named in the doc comment
- `parse_positive_integer(s: &str) -> Option<u32>` — **return `Option`, not `0`**.
  Doc comment: *"Returning `0` on failure would be the old way. `Option` is Rust's
  answer to that pattern; you'll explore it fully in chapter 5."*

**Fix from audit:**
- `calculate_damage` test must include a case where truncation and rounding diverge
  (e.g. input that produces `x.6` → truncates to `x`, rounds to `x+1`) so only one
  implementation passes.

---

### Chapter 1 · Owned or Borrowed
*Difficulty: Easy*

**Opening hook:** Most languages treat strings as either bytes or characters and let
you confuse the two. Rust has two string types on purpose: `&str` borrows data that
lives elsewhere; `String` owns it. This distinction — ownership — is the idea that
makes Rust memory-safe. Here is the 30-second version.

**Spiral 1 — plant the mental model here:**
Add a section to the intro titled *"The 30-second ownership preview"*. State the three
rules plainly, without proof:
1. Every value has exactly one owner.
2. When the owner goes out of scope, the value is dropped.
3. You can borrow a value — `&T` to read, `&mut T` to modify — without taking
   ownership.

Then: *"You don't need to fully understand these yet. But every time you see `&` in a
type signature for the rest of the course, this is why it's there. Chapter 9 is where
we put it all together."*

**Concepts:** `&str` vs `String`, `.len()` (bytes) vs `.chars().count()` (characters),
`.to_uppercase()`, `.chars().next()` returning `Option<char>` (a sneak preview).

**Exercises:** keep the current strings exercises unchanged.
- `count_chars(text: &str) -> usize`
- `shout(text: &str) -> String`
- `has_uppercase(text: &str) -> bool`
- `first_char(text: &str) -> Option<char>`

---

### Chapter 2 · Control Flow
*Difficulty: Easy*

**Opening hook:** Rust's control flow is unsurprising by design — `if`, `while`,
`for`, `loop`. The one thing worth calling out: `match` requires exhaustive handling.
The compiler will not let you add a new variant to an enum and forget to handle it
somewhere. (You'll see this pay off in chapter 7.)

**Concepts:** `if`/`else` as expressions, `for` over ranges and iterators, `while`,
`loop` with `break value`, `continue`.

**Exercises:** keep current. No structural changes needed.

---

### Chapter 3 · Functions as Contracts
*Difficulty: Easy*

**Opening hook:** In Rust, every parameter type and return type is written out
explicitly. There are no implicit conversions, no optional arguments without a type,
no `any`. A function signature is a binding contract — the compiler enforces both
sides.

**Spiral 2 — first move error, on purpose:**
Add one exercise or experiment that intentionally shows a move error. The learner
passes a `String` to a function, then tries to use the original binding afterward.
The exercise asks them to fix it — either by passing `&str` instead, or by cloning.
This must be structured and required, not a commented-out optional.

Suggested addition after the regular exercises:

```rust
// This doesn't compile. Why? Fix it two ways:
// 1. Change the parameter type so ownership is not transferred.
// 2. Keep the parameter type but give the function its own copy.
fn takes_string(s: String) { println!("{s}"); }

fn main() {
    let greeting = String::from("hello");
    takes_string(greeting);
    println!("{greeting}"); // error: borrow of moved value
}
```

**Exercises:** keep current functions exercises, plus the above structured experiment.

---

### Chapter 4 · Functions as Values
*Difficulty: Moderate* · **New chapter — write from scratch**

**Opening hook:** A closure is a function you can pass around like a value. It can
capture variables from the surrounding scope. That's it. Closures are the building
block of every iterator chain, every `Option::map`, every callback you'll write in
Rust — so they get their own chapter before you need them everywhere else.

**Why this chapter must exist:**
Closures are currently introduced as a footnote in the `Option` chapter and again in
the iterators chapter. Learners encounter `|s| s.len()` without ever having a chapter
where closures are the point. This produces confusion in every subsequent chapter.

**Concepts:**
- Basic syntax: `|x| x + 1`, `|x: i32| -> i32 { x + 1 }`
- Multi-statement closures: `|x| { let y = x * 2; y + 1 }`
- Capturing from environment: immutable capture, mutable capture with `mut`
- `move` closures: taking ownership of captured values
- When to annotate types vs. let them be inferred
- The three closure traits (`Fn`, `FnMut`, `FnOnce`) — briefly, as "the compiler picks
  the right one; you'll see these in trait bounds later"

**Exercises:**
- `apply(f: impl Fn(i32) -> i32, x: i32) -> i32` — call a closure once
- `apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32` — compose
- `make_adder(n: i32) -> impl Fn(i32) -> i32` — return a closure (capturing `n`)
- `filter_positive(numbers: Vec<i32>) -> Vec<i32>` — use a closure with `retain` or
  manual filter loop
- `transform_all(words: Vec<String>, f: impl Fn(String) -> String) -> Vec<String>` —
  map with a closure

---

### Chapter 5 · The Billion-Dollar Mistake, Fixed
*Difficulty: Moderate*

**Opening hook:** In 1965, Tony Hoare introduced the null reference. He later called
it his "billion-dollar mistake": the source of countless crashes, vulnerabilities, and
production incidents across every language that inherited it. Rust has no null. Instead,
a value that might be absent has type `Option<T>` — and the compiler forces you to
handle both cases.

**Concepts:** `Option<T>` (`Some`/`None`), `match`, `if let`, `unwrap_or`,
`unwrap_or_else`, `map`, `and_then`. Explicitly name `unwrap` as the escape hatch that
re-introduces the possibility of a crash — *"use it in tests or when you've already
ruled out `None`; otherwise reach for the combinators."*

**Prerequisite payoff:** Closures (ch.4) are now available, so `map(|x| ...)` and
`and_then(|x| ...)` can be introduced without a detour.

**Exercises:** keep current.
- `get_setting_or_default(setting: Option<u32>, default: u32) -> u32`
- `transform`: `Option<i32>` → double the inner value, or `None`
- `first_char(text: &str) -> Option<char>`
- `find_user_by_id(users: &[(u32, String)], id: u32) -> Option<&str>`

---

### Chapter 6 · Errors You Cannot Ignore
*Difficulty: Moderate*

**Opening hook:** Most languages let you ignore errors. A function returns an error
code; the caller discards the return value. An exception is thrown; no `catch` is
required. The result is silent failures, corrupted state, and production incidents that
trace back to an error that was never handled. In Rust, a function that can fail says
so in its return type. The compiler ensures you deal with it.

**`?` operator is taught here, not in a separate chapter 8 chapters later.**

Structure:
1. Introduce `Result<T, E>` and the verbose `match` form. Write one exercise in this
   style so learners feel the pain.
2. Introduce `?` as the shorthand. Rewrite the same exercise. The difference is the
   lesson.
3. Introduce `Box<dyn Error>` for mixing error types, briefly.

**Cognitive load fix from audit:** The current intro bundles `Result` with `&'static
str`, turbofish syntax, and match guards all at once. In the new version:
- Core concept first: `Result<T, E>`, `Ok`, `Err`, `match`.
- Move `&'static str`, turbofish, and match guards into a collapsible **"Syntax you'll
  see in these exercises"** aside that learners can consult without it interrupting the
  main explanation.

**Fix test races from audit:** These exercises write real files. Either use unique
per-test filenames (e.g. derived from a constant unique to each test) or add
`tempfile` as a dev-dependency. Document the choice. Do not leave `--test-threads=1`
as the only mitigation.

**Exercises:**
- `safe_divide(a: f64, b: f64) -> Result<f64, &'static str>`
- `read_config(path: &str) -> Result<String, io::Error>` — using `?`
- `validate_email(email: &str) -> Result<&str, &'static str>`
- `parse_percentage(input: &str) -> Result<u8, &'static str>` — the hard one
- `chain_operations(path: &str) -> Result<u8, Box<dyn Error>>` — new; chains
  `read_config` and `parse_percentage` together using `?`, demonstrating mixed error
  types

---

### Chapter 7 · Making Impossible States Unrepresentable
*Difficulty: Moderate*

**Opening hook:** In most type systems, you can construct values that should never
exist. A `User` with a non-null `email` but an `account_type` that says
`"unregistered"`. An HTTP response with both a body and a `204 No Content` status.
Rust's enums carry data. A well-designed enum makes the impossible states simply
unrepresentable: the type system rejects them at compile time.

**Concepts:** Enums with data-carrying variants, `match` with destructuring, nested
`Option`/`Result` variants, `if let`, wildcard `_`, multiple patterns `A | B`.

**`#[derive]` explained here:** Add to intro: *"`#[derive(Debug, PartialEq)]` is
shorthand for 'please write the obvious `impl Debug for ...` and `impl PartialEq for
...` blocks for me.' A trait is a shared interface — Rust's word for what Java calls
an interface or what Go calls a protocol. Chapter 10 is the traits chapter; for now,
just know that `derive` saves you from writing boilerplate."*

**Fix `HttpStatus` move error from audit:** `HttpStatus` is not `Copy`, so passing it
to two functions in sequence fails. Make this intentional: derive `Copy, Clone` on
`HttpStatus`, then add a prose note explaining why — *"simple enums with no heap data
can be `Copy`; the compiler duplicates them for free, just like integers."*

**Exercises:** keep current enums exercises, adjusted for the above.

---

### Chapter 8 · Structured Data Without the Baggage
*Difficulty: Moderate*

**Opening hook:** Object-oriented languages bundle data and behavior together through
classes and inheritance. The inheritance part causes problems: deep hierarchies are
fragile, and the "gorilla-banana-jungle" problem (you wanted a banana, you got the
gorilla holding the banana, and the entire jungle) is a common maintenance trap. Rust
has structs for data and `impl` blocks for behavior. No inheritance. Composition
instead.

**Spiral 3 — `&self` / `&mut self` as parameter ownership:**
The intro must make explicit: *"Method receivers are just parameters. `&self` is a
borrowed reference to the struct (read-only). `&mut self` is a mutable borrow (you can
change fields). `self` without `&` moves the struct into the method (it's consumed).
You've already seen all three forms on regular functions in chapter 3."*

**Fix from audit:** `record_login` currently sets `is_verified = true` on every call,
which is correct by spec but the spec is ambiguous ("marks as verified after first
login"). Either rename to `mark_verified_and_record_login`, or add a test showing
idempotent behavior is intentional.

**Exercises:** keep current structs exercises.

---

### Chapter 9 · Memory Safety Without a Garbage Collector
*Difficulty: Spike*

**Opening hook:** Every memory safety CVE in Chrome, Windows, and the Linux kernel —
use-after-free, buffer overflow, double-free — traces back to the same root cause:
the programmer is responsible for manually tracking when memory is safe to use and
when to free it. Garbage collectors automate this but add runtime overhead and
unpredictable pauses. Rust does neither. It proves memory safety at compile time,
for free, using three rules you've already been using since chapter 1.

**This is the spiral payoff.** By now learners have seen:
- Copy types (ch.0): "integers duplicate for free"
- `&str` vs `String` (ch.1): "&str borrows, String owns"
- Move on function call (ch.3): "passing a String to a function moves it"
- `&self` / `&mut self` (ch.8): "methods borrow the struct"

The chapter opens: *"You've been using the ownership system for nine chapters. Here
are the three rules that explain everything you've seen. None of them will surprise you."*

**Make the experiments mandatory.** The current experiments file (commented-out code
to uncomment) is too easy to skip. Restructure as a required exercise step:
- Each experiment is its own numbered exercise file, not a comment.
- The exercise `.rs` file contains intentionally broken code (not commented out).
- The task is: read the compiler error, fix the code, describe in a `// ANSWER:` comment
  what rule was violated. Tests check that the fixed code compiles and produces the
  right output.
- This makes it impossible to pass without actually triggering and reading the error.

**Three required experiments:**
1. Use after move: `let s = String::from("hi"); let t = s; println!("{s}");`
2. Two simultaneous mutable borrows
3. Mutable borrow while a shared borrow is live

**Difficulty signpost:** Add to dashboard and chapter intro: *"This is the first of
two hard chapters. Budget extra time. If the borrow checker confuses you here, that
is normal and expected — it confuses everyone the first time. The exercises are
designed to make the errors visible, not to frustrate you."*

**Exercises:** keep current ownership exercises + restructured experiments above.

---

### Chapter 10 · Shared Behavior Without Inheritance
*Difficulty: Moderate*

**Opening hook:** Inheritance promises reuse but delivers coupling. Deep class
hierarchies break when requirements change, because a change to a base class ripples
everywhere. Rust has no inheritance. Instead, traits define shared behavior that any
type can opt into independently. Composition over inheritance, enforced by the type
system.

**Concepts:** Defining a trait, implementing a trait, `Display`, default methods,
trait bounds (`fn f<T: Trait>(x: T)`), trait objects (`dyn Trait`), `impl Trait` in
return position, the standard-library trait table (connect back to `Debug`,
`PartialEq`, `Clone`, `Copy`, `From`, `Into` — all of which they've been using).

**Exercises:** keep current traits exercises.

---

### Chapter 11 · Zero-Cost Abstractions
*Difficulty: Hard*

**Opening hook:** In most languages, higher-order functions — map, filter, fold — are
convenient but slower than hand-written loops, because each step allocates an
intermediate collection. Rust's iterators are lazy and fused by the compiler into a
single loop. The abstraction costs nothing at runtime. This is the payoff for a strict
type system: you can write elegant, high-level code and get C-speed output.

**Prerequisite payoff:** Closures (ch.4) are already known. There is no sidebar
needed.

**`&&T` table in cheatsheet — required.** Add this table to the cheatsheet and link
to it from this chapter's intro:

| Source | `.iter()` yields | `.into_iter()` yields |
|---|---|---|
| `Vec<T>` | `&T` | `T` |
| `&[T]` | `&T` | `&T` |
| `&[&str]` | `&&str` | `&&str` |

Add: *"Inside `.filter` and `.find`, the closure receives a reference to each item.
If the item is itself a reference, you get a double reference — `&&str`. Use
`.copied()`, `|&&x|`, or explicit dereference to peel a layer."*

**Difficulty signpost:** Add to intro: *"This is the second hard chapter.
The concept is simple; the friction comes from the type-system bookkeeping around
references in closures. When you hit a type error, check the `&&T` table in the
cheatsheet first."*

**Exercises:** keep current iterators exercises. Remove or defer the lifetime
annotation in `find_rust_files` — lifetimes are not the lesson here.

---

### Chapter 12 · Heap Memory, Under Control
*Difficulty: Moderate*

**Opening hook:** In C, every `malloc` needs a matching `free`. Miss one and you have
a memory leak. Double-free the same pointer and you have undefined behavior that an
attacker can exploit. Rust's smart pointers solve this with ownership: the memory is
freed exactly when the owning value goes out of scope, with no garbage collector, and
no manual `free`.

**Concepts:** `Box<T>` (heap allocation, single owner), recursive types (why `Box` is
required), `Box<dyn Trait>` (owned trait objects, connecting back to ch.10), `Rc<T>`
(shared ownership, single-threaded).

**Add an `Rc<T>` exercise from audit.** Currently `Rc` is described in depth but has
no exercise. Minimum viable exercise:

```rust
// Build a simple shared configuration that two "components" both hold a reference to.
// Use Rc::clone to share ownership. Print the reference count at key moments.
fn shared_config() {
    use std::rc::Rc;
    let config = Rc::new(vec!["debug=true", "port=8080"]);
    let component_a = Rc::clone(&config);
    let component_b = Rc::clone(&config);
    // assert_eq!(Rc::strong_count(&config), 3);
    todo!()
}
```

`RefCell<T>` — mention in one paragraph with a pointer to further reading, but no
exercise. It pairs with `Rc` for graph-shaped data and testing patterns; not needed
for the chapters ahead.

**Exercises:** keep current smart pointers exercises + the `Rc` exercise above.

---

### Chapter 13 · Fearless Concurrency
*Difficulty: Hard* · **New chapter — write from scratch**

**Opening hook:** Data races are among the hardest bugs to reproduce and debug.
Two threads write to the same memory. Most runs are fine. One run in ten thousand
crashes. The OS gives you no useful error. Debuggers are nearly useless because the
bug disappears under observation. Languages that don't check for data races at compile
time — which is almost all of them — leave you to find these at 3am in production.
Rust's borrow checker makes data races a compile error.

**This chapter delivers on the fourth promise made in "Why Rust?"**

**Concepts and structure:**
1. `thread::spawn` and `thread::join` — basic thread creation.
2. Show a data race: two threads mutating a shared `Vec` without synchronization.
   The compiler refuses. Show the error message. Explain why.
3. `Arc<T>` (thread-safe shared ownership): `Rc` won't compile across threads; show
   why, then fix it with `Arc`.
4. `Mutex<T>` (interior mutability with compile-time deadlock resistance): wrap the
   shared data in `Arc<Mutex<T>>`, lock before access, show that forgetting the lock
   is a type error.
5. `mpsc::channel`: message passing as an alternative to shared state. Brief; enough
   to show the pattern.
6. The payoff line: *"The same borrow checker that prevents use-after-free in single-
   threaded code prevents data races in concurrent code. It is the same mechanism,
   applied to threads."*

**Exercises:**
- `sum_in_parallel(numbers: Vec<i32>) -> i32` — split a slice, spawn threads, join
  and sum. Requires `Arc` for the shared data.
- `concurrent_counter() -> u32` — increment a counter from two threads using
  `Arc<Mutex<u32>>`. Return the final value.
- `pipeline(items: Vec<String>) -> Vec<String>` — send items through an `mpsc`
  channel, transform in a worker thread, collect results.

---

### Chapter 14 · Visibility at Scale
*Difficulty: Easy*

**Opening hook:** As programs grow, "everything is accessible from everywhere" becomes
a maintenance problem. Rust's module system defaults to private: nothing is visible
outside its module unless explicitly marked `pub`. This makes refactoring safe — if
you can change the internals of a module without the compiler complaining, nothing
outside was relying on them.

**Concepts:** `mod`, `pub`, `pub(crate)`, `use`, nested modules, `super`, the
difference between `pub` on a struct and `pub` on its fields.

**Exercises:** keep current modules exercises.

---

### Chapter 15 · Project: Word Frequencies
*Difficulty: Moderate (synthesis)*

**Frame this as a synthesis project, not just another chapter.** The intro should
explicitly say: *"This is the first of three project chapters. There is no new
concept. The goal is to combine iterators, `HashMap`, and `Option` the way you would
in a real program. The functions here are more open-ended than previous exercises."*

**Fix from audit:**
- `most_common_word`: add hint *"`into_iter()` on a `HashMap` gives you `(K, V)`
  pairs by value; that's what makes returning `(String, usize)` possible here."*
- `text_stats`: add one sentence acknowledging the tuple return type: *"In production
  code you'd define a `TextStats` struct; we use a tuple to keep the focus on the
  iterator chain."*

**Exercises:** keep current word-frequencies exercises.

---

### Chapter 16 · Project: Environment Parser
*Difficulty: Hard (synthesis)*

**Frame as above.** This is the first project that requires proper error propagation
with `?`. By now learners have had `Result` and `?` since chapter 6, so this is
application, not introduction.

**Fix from audit:**
- Clarify what `parse_env_file` does on a malformed line: stop and return `Err`.
  Add doc comment: *"Strict parsing: stop at the first malformed line and return `Err`.
  This is easier to debug than silently skipping lines."*
- `get_env_var<T: FromStr>`: add a hint explicitly recommending `.ok()` to discard the
  parse error, explaining why `?` won't work here without a `From` bound on `T::Err`.

**Exercises:** keep current env parser exercises.

---

### Chapter 17 · Project: CSV Parser
*Difficulty: Spike (synthesis)*

**This is the hardest chapter. Announce it.**

Add to intro: *"This is the hardest project in the course. `parse_simple_csv_line` is
easy; `parse_csv_line` is a small state machine implementing a subset of RFC 4180. If
you find it hard, you are finding it appropriately hard. Budget time."*

**Fix the difficulty cliff in `parse_csv_line` from audit:**

1. Add pseudocode to the intro (not Rust, just logic):
   ```
   state = outside_quote
   for each char:
     if outside_quote:
       '"'  → state = inside_quote
       ','  → push current field, start new field
       else → append to current field
     if inside_quote:
       '"'  → peek next char:
                '"' → append literal '"', advance past both
                else → state = outside_quote
       else → append to current field
   after loop: push final field
   ```

2. Add an intermediate test between the trivial case and the full RFC case:
   `r#"a,"b",c"#` → `["a", "b", "c"]` (quote-stripping, no embedded commas or
   doubled quotes). This creates a three-rung ladder instead of a two-rung cliff.

**Exercises:** keep current CSV exercises with above additions.

---

### Appendix · How This Course Is Designed

**Purpose:** Honest statement of pedagogical philosophy, usable as marketing. Frame it
as the result of years of teaching Rust in consulting engagements, not as a critique
of any specific resource.

**Structure:**

> ### Why we built this
> I've helped many engineering teams adopt Rust professionally. Over those engagements
> I kept hitting the same pattern: developers with strong backgrounds — Python, Go,
> Java, C++ — would stall at the same places, for the same reasons. The available
> learning resources are excellent references, but they were designed to be complete,
> not to be fast. This course is designed around the things that actually cause people
> to stall.

Then a table of design decisions (not framed as "X is bad", framed as "here is what
we chose and why"):

| Design decision | What it solves |
|---|---|
| Exercises from the first chapter | Learning to write code requires writing code, not reading about it |
| Motivation before mechanics | Learners who don't know *why* a concept exists forget it faster |
| Spiral ownership introduction | Ownership as a cliff causes learners to freeze; as a thread, it clicks |
| `Result` and `?` taught together | Writing verbose error handling for 8 chapters builds bad habits |
| Closures before `Option` and iterators | Higher-order methods require closures; prerequisites must come first |
| Explicit difficulty labels on every chapter | Struggling with the hard chapters is normal, not a sign of failure |
| Fearless concurrency as a first-class chapter | It is one of Rust's core promises; a course that doesn't deliver on it is incomplete |

---

## Cross-cutting changes

### Difficulty labels on everything
Every chapter in the chapter list, every exercise within a chapter, carries a
difficulty label. Use a small badge in the UI: `○` Trivial, `◔` Easy, `◑` Moderate,
`◕` Hard, `●` Spike. The label appears before the learner opens the chapter.

### Hints file for every chapter
Every chapter must have a `2_hints.md`. One paragraph per exercise function: not the
answer, but the method or pattern to reach for. This is the difference between "stuck
for 5 minutes" and "stuck for 45 minutes and now demoralized."

### Cheatsheet additions
- The `&&T` / iterator-yield table described in chapter 11.
- A "when to use which string type" table: `&str` for read-only parameters, `String`
  for owned data, `&String` almost never.
- A "which closure trait am I?" decision tree: `FnOnce` if it consumes a capture,
  `FnMut` if it mutates one, `Fn` otherwise.

### Dashboard / UI
- Difficulty badge visible on every chapter card before it is opened.
- After the chapter 9 (Ownership spike) chapter card, add a callout:
  *"This is the hardest concept in the course. If it takes you longer than expected,
  that is expected."*
- `todo!()` panic output should be intercepted and displayed as:
  *"Looks like this function still has `todo!()` in it. Replace it with your
  implementation and run again."* instead of the raw `not yet implemented` message.

---

## New content required

The following does not exist yet and must be written from scratch:

1. **"Why Rust?" pre-course page** (read-only, ~800 words, see spec above)
2. **Chapter 4: Functions as Values** (closures, see spec above)
3. **Chapter 13: Fearless Concurrency** (threads, `Arc`, `Mutex`, channels, see spec
   above)
4. **Appendix: How This Course Is Designed** (see spec above)

The concurrency chapter is the largest new investment. Scope it as three exercises
(parallel sum, concurrent counter, pipeline) — enough to demonstrate the compile-time
data-race guarantee without turning into an async/await course.

---

## Deferred work

**Language-personalized motivating examples.** The problem-first framing (show the
bug in the language the learner knows before showing how Rust fixes it) is the right
long-term direction. Deferred because it requires maintaining separate example sets for
Python, JavaScript, Java, Go, and C/C++, and that scope is separate from the core
restructure.

When this is picked up: implement as a language toggle (cookie or URL param) on the
motivating hook section of each pillar chapter (chapters 5, 6, 9, 13). The rest of
the chapter stays the same; only the 2–3 sentence opening example changes.

Priority order for language examples: Python first (largest audience), then
JavaScript, then Go, then Java/C#, then C/C++.

---

## Items absorbed from new5.md

The following were in the earlier audit (new5.md) and are still valid. They are
incorporated into the chapter specs above but listed here for traceability:

- `parse_positive_integer` → `Option<u32>` return type (ch.0)
- Tax test: add a case that distinguishes truncation from rounding (ch.0)
- `#[derive]` explanation as trait shorthand (ch.7)
- `HttpStatus` must be `Copy` or the move error must be intentional (ch.7)
- `record_login` idempotence ambiguity (ch.8)
- Result intro: cognitive load split into main concept + syntax aside (ch.6)
- `Rc<T>` exercise added (ch.12)
- `&&T` table in cheatsheet (ch.11)
- Ownership experiments made mandatory and non-skippable (ch.9)
- Difficulty cliff warning on dashboard and in ch.9 and ch.11 intros
- Test-race bug in `?` operator exercises fixed with unique filenames or `tempfile` (ch.6)
- `hints.md` for every chapter
- `most_common_word` hint about `.into_iter()` (ch.15)
- `text_stats` tuple-return acknowledgment (ch.15)
- `parse_csv_line` pseudocode added to intro (ch.17)
- `parse_csv_line` intermediate test added (ch.17)
- `todo!()` panic message replaced with friendly copy (UI)
