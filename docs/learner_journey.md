# The learner's mental journey

A chapter-by-chapter pass through every exercise from the perspective of
someone who has *just* finished the previous one. Each entry has:

- **State of mind** — what they're carrying in (energy, fears, prior
  context).
- **Inner monologue** — stream-of-consciousness as they read the file.
- **Wins** — the tiny "ah!" moments to engineer for.
- **Sharp edges** — what's likely to actually trip them up, with
  concrete suggestions.
- **Complexity** — `Trivial`, `Easy`, `Moderate`, `Hard`, `Spike`.
  "Spike" means the chapter introduces a real difficulty cliff
  compared to the one before it.

Complexity is rated *for the audience this chapter assumes*, not in
absolute terms. Chapter 8 isn't harder than 17 in absolute terms, but
relative to its predecessor it's a bigger leap.

---

## 00 — Hello, Rust!  ·  *Trivial*

**State of mind.** Excited, slightly nervous. They've installed the
toolchain, the website worked, they hit Run on something for the first
time. They want a quick win to confirm "yes, I can do this."

**Inner monologue.** "OK, `format_welcome_message`, return
`'Welcome, {name}!'`. I've seen `format!` in the cheatsheet... is that
the right one? Let me try `format!("Welcome, {name}!")`. Run. Green.
That was nice."

**Wins.**
- The whole loop (edit → Run → green tick → Submit) clicks in under a
  minute.
- They notice `format!` works with the `{name}` shorthand and feel
  modern.

**Sharp edges.**
- Some learners will type `println!` instead of `format!` because the
  intro emphasises printing. Worth a one-liner in the doc comment:
  *"This function returns a `String`; it doesn't print."*
- The `todo!("replace this line with your code")` macro will panic with
  a confusing-looking message if they hit Run before editing. Consider
  checking that the run-status copy degrades gracefully on
  `not yet implemented` panics ("Looks like you haven't written the
  function body yet.").

---

## 01 — Integer handling  ·  *Easy*

**State of mind.** Confident from chapter 0. Three functions instead
of one — they read it as "three of the same thing".

**Inner monologue.** "Number to string, just `n.to_string()`. Tax: cast
to `f64`, multiply, cast back. Wait, do I round or truncate? The test
says `100 * 1.085 = 108`, so... rounding? Let me try `as u32`. Test
says `108`, which is `108.5` rounded down? Or up? Oh, `as u32` on a
positive `f64` truncates, so `108.5 as u32 = 108`. Phew, lucky."

**Wins.**
- `to_string()` is satisfyingly obvious.
- The "test gave me the right answer by luck" moment for tax is a
  real teaching opportunity if the prose flags it.

**Sharp edges.**
- The tax test (`50 * 1.10 = 55.0` exactly, `100 * 1.085 = 108.5`)
  passes with `as u32` truncation, but only because both inputs round
  the "right" way. A learner who explicitly calls `.round()` will also
  pass. **Both implementations are accepted but they differ on inputs
  the tests don't cover.** Either add a third assertion that
  distinguishes them (e.g. `(100, 8.4)` → `108`), or call out in the
  doc comment that "we accept either truncation or rounding here."
- `parse_positive_integer` returns `0` on failure, which is exactly
  the anti-pattern Rust is famous for fixing. Fine as an introductory
  exercise but worth one sentence in the prose: *"Returning `0` on
  failure is a placeholder design. Chapter 7 introduces `Option`,
  which is what you'd actually reach for."*
- The negative-number case (`"-5"` → `0`) is subtle: `"-5".parse::<u32>()`
  fails because `u32` can't be negative. A learner who tries
  `i32` first will get unexpected behaviour. The chapter intro should
  mention this in passing.

---

## 02 — Strings and chars  ·  *Easy*

**State of mind.** Still riding the wave. The intro table
("borrowed/owned") feels reassuring — there's a *system* here.

**Inner monologue.** "`count_chars` — the comment literally says
`.len()` is bytes, use `.chars()`. Easy: `text.chars().count()`.
`shout` — `to_uppercase()` returns... `String`? Yes. Done.
`has_uppercase` — `.any()`, the doc link is right there.
`first_char` — `chars().next()` returns `Option<char>` already, so
literally one line."

**Wins.**
- The doc-link hints land perfectly. The learner *uses* the docs,
  which is the real skill.
- They get a sneak peek of `Option` (`first_char`) without it being
  scary — the iterator already produces one.

**Sharp edges.**
- `text.chars().count()` is `O(n)`. Some learners will worry about
  performance. A throwaway sentence in the prose ("yes, this walks the
  string — that's the cost of UTF-8") settles it.
- The `has_uppercase` exercise quietly assumes ASCII. That's why it
  says "ASCII uppercase" in the comment. Good.

---

## 03 — Enums and matching  ·  *Easy*

**State of mind.** First proper Rust idiom. The intro mentions
"exhaustive matching" — they're curious whether the compiler really
will yell at them.

**Inner monologue.** "Five variants, five arms. Done. `should_retry`:
only `InternalServerError` retries — I can either match all five and
return `false` for four of them, or... use `_`. The intro said
'exhaustive', so the compiler is OK with `_`? Let me try. Yep. So
`_` *counts as* exhaustive. Got it."

**Wins.**
- Triggering an exhaustiveness error on purpose (omit one variant)
  and seeing the compiler complain is a great experience. Worth
  suggesting in the prose: *"Try deleting one of the arms and
  hitting Format — the compiler will tell you exactly which case
  you missed."*

**Sharp edges.**
- The `_` wildcard vs. listing variants explicitly is a real design
  question. A short note on "I prefer to list variants until the list
  gets unwieldy, because then the compiler reminds me to handle new
  ones I add" would seed good habits.
- `HttpStatus` is moved by value into `status_code` and `should_retry`,
  which is fine for a `Copy`-shaped enum but they're not getting
  `Copy` here. The first time they call `status_code(s)` then
  `should_retry(s)` they'll hit a move error. Either derive `Copy` on
  the enum, or *intentionally* leave it off and add an exercise prompt
  *"Try calling both functions on the same value — what does the
  compiler say?"*

---

## 04 — Vectors basics  ·  *Easy*

**State of mind.** Comfortable. They've been waiting for "a list".

**Inner monologue.** "`count_items`: `.len()`. `add_item`: `.push()`.
`contains_item`: `.contains(item)`... wait, type mismatch. `contains`
wants `&String` but I have `&str`. Hmm. Let me Google. Oh,
`.iter().any(|x| x == item)` works. Or `.contains(&item.to_string())`,
but that allocates."

**Wins.**
- `add_item` mutating through `&mut Vec<String>` is the first time
  they see the system *encourage* mutation through a reference.
- `create_shopping_list` cementing "borrowed in, owned out" matches
  the intro's table.

**Sharp edges.**
- **`Vec::contains` vs `&str` is a sharp edge in the wild.** The
  signature is `fn contains(&self, x: &T) -> bool`, which here means
  `&String`. The most ergonomic real-world fix is
  `.iter().any(|s| s == item)`, but iterators haven't been formally
  introduced yet (chapter 11). The doc comment should *explicitly*
  acknowledge this: *"You'll find that `Vec::contains` doesn't quite
  fit; a `for` loop with `if name == item { return true; }` is the
  most direct way at this point."*
- `&Vec<String>` vs `&[String]` — clippy will eventually nag about
  this. Worth a single sentence in the intro: *"`&Vec<String>` is
  fine here for clarity; `&[String]` is the more idiomatic
  signature, and you'll see it in chapter 11."*
- `create_shopping_list(items: &[&str])` is a triple-indirection
  signature that some learners will stare at for a while. A short
  example in the prose ("`&[&str]` is a borrowed slice of borrowed
  string slices, like `&["bread", "milk"]`") would help.

---

## 05 — HashMaps  ·  *Easy*  (with one moderate function)

**State of mind.** They feel like they know "real" Rust now. `Vec`
and `HashMap` are the two collections every working program uses.

**Inner monologue.** "`create_default_config` — fine, two inserts.
`set_config_value` — one insert, same call. `get_config_value` —
match on `Option`, return `value.clone()` or `'default'.to_string()`.
`count_words` — oh, `.entry().or_insert(0)` is the canonical move,
but I don't know that yet. Let me do `if contains_key { ... } else { ... }`."

**Wins.**
- `create_default_config` printing is a tiny but real
  *"I built a thing!"* moment.
- `count_words` is genuinely satisfying when they discover
  `.entry(...).or_insert(0)` (or the prose nudges them there).

**Sharp edges.**
- `count_words` is *the* moment to formally introduce
  `entry`/`or_insert`. The current doc comment hints at it; if the
  intro markdown shows the pattern explicitly, learners save 10
  minutes of fighting borrow errors with `if contains_key { *m.get_mut(k).unwrap() += 1 }`.
- **Subtle borrowing bug they'll hit:** they may try
  `for word in words { ... map.insert(word.to_string(), ...) }` and
  it works. Then they try
  `for word in words { let count = map.get(word).unwrap_or(&0); map.insert(...) }`
  and it doesn't, because `get` returns a reference that conflicts
  with the next `insert`. The intro should warn that "fetching a
  reference into the map and then mutating the map can't both happen
  at once — `entry` exists exactly because of that."
- `get_config_value` returning `String` forces a clone or
  `to_string()`. Some learners will return `&str` and hit a lifetime
  error. Worth a sentence: *"Returning `String` here costs a clone
  but keeps the signature simple. In chapter 9 you'll see when
  borrowing the value out instead is feasible."*

---

## 06 — Tuples  ·  *Trivial*

**State of mind.** Refreshing palate-cleanser after `HashMap`.

**Inner monologue.** "Return `(String, u32)`, fine. Destructure with
`let (a, b) = ...`. Swap is one-liner. Done in five minutes."

**Wins.**
- The "swap two values" with `(b, a)` in *one expression* makes
  C/C++/Java refugees grin.
- Pattern destructuring in `let` previews what `match` patterns can
  do.

**Sharp edges.**
- `get_first_name(full_name: (String, String))` *moves* the tuple.
  After the call, the original is gone. Probably fine — they haven't
  met ownership formally yet — but a learner who reuses `full_name`
  later in a print statement will get their first move error here. A
  one-line note in the chapter intro ("we cover what 'moved' means
  in chapter 9") softens the surprise.
- `swap_values((1, 2))` taking the tuple by value is fine because
  `(i32, i32)` is `Copy`. The asymmetry with `get_first_name` (not
  `Copy` because of `String`) will confuse the keen ones. Could be a
  sidebar in the intro.

---

## 07 — Option handling  ·  *Moderate*

**State of mind.** They know the *shape* of `Option` from chapter 2
(`first_char`). Now they're being asked to actually *manipulate*
one.

**Inner monologue.** "`get_setting_or_default` — that's literally
`unwrap_or`. `optional_string_length` — `.map(|s| s.len()).unwrap_or(0)`.
Or a `match`. `get_first_item` — `slice::first()`, the comment
basically says so. `find_user_by_id` — iter, find, map. The find
returns `Option<&(u32, String)>`. Then I `.map(|(_, name)| name.as_str())`?
Or `name.as_ref()`? Wait, what's the difference?"

**Wins.**
- The four-step ladder (consume → consume + transform → produce →
  produce by searching) is a *great* pedagogical structure. They
  feel the difficulty climb and they notice it.
- Discovering `unwrap_or` and `map` makes their match-heavy first
  attempts feel verbose. They'll start reaching for combinators on
  their own.

**Sharp edges.**
- `find_user_by_id`'s return type is `Option<&str>`, but the iterator
  yields `Option<&(u32, String)>`. The type gymnastics
  (`map(|(_, name)| name.as_str())`) is a genuine puzzle. Worth a
  worked-out hint in the intro showing the chain step by step,
  *with the type annotations*.
- `get_first_item` returning `Option<&String>` (not
  `Option<&str>`) is technically a slightly worse signature than the
  more idiomatic `Option<&str>` — but going to `&str` requires
  `.map(String::as_str)`, which the chapter hasn't taught yet. Fine
  to leave, but call out in the prose: *"`Option<&String>` is fine
  here; chapter 11 shows how `.map(String::as_str)` would let you
  return the more idiomatic `Option<&str>`."*
- This is the first chapter where they'll seriously want closures
  (`.map(|x| ...)`). The intro should formally introduce the
  `|x| body` syntax — right now it's relying on osmosis.

---

## 08 — Result handling  ·  *Moderate*

**State of mind.** "Oh, like Option but with an error." They
*think* they know what they're in for.

**Inner monologue.** "`safe_divide`: `if b == 0.0 { Err(...) } else
{ Ok(a / b) }`. Easy. `read_config_file`: same shape with `String::from`.
`validate_email`: `if email.contains('@') { Ok(email) } else { Err(...) }`.
`parse_percentage`: hmm, three failure modes. Strip `%`, parse, check
range. Three `if`s? Or chain with `?`... wait, `?` is next chapter.
OK, nested `match`."

**Wins.**
- The `parse_percentage` exercise (now that it's not the
  spoiled `parse_port`) is a small genuine puzzle. The prose
  *describes* the strategy without giving the code.
- They notice that `Ok` and `Err` types are independent, which
  unlocks "I can have a numeric error type but a string success" in
  their mental model.

**Sharp edges.**
- `validate_email` returning `Result<&str, &'static str>` introduces
  *implicit lifetime elision*. They won't know that's what's
  happening; they'll just notice the function compiles and not ask.
  A two-sentence note: *"The `&str` in the return type borrows from
  `email` — the compiler infers the lifetime. Chapter 9 makes this
  explicit."*
- `parse_percentage` is genuinely the hardest function in the
  chapter and lives at the bottom. A learner who blasts through the
  first three and then stalls here may feel like they're stupid
  rather than tackling the deliberately harder one. Worth a tiny
  marker in the doc comment ("This one's the harder one — the
  previous three are warmups").
- The error type `&'static str` is fine for now, but learners who
  try to put a `format!`-built message in there will hit the
  `String` vs `&'static str` wall. The intro already acknowledges
  this; consider one more line: *"If you find yourself wanting
  `format!(...)` in an `Err`, change the error type to `String`."*

---

## 09 — Ownership basics  ·  *Spike*

**State of mind.** They've heard ownership is the hard part. They're
nervous. They want it to make sense, not just memorise it.

**Inner monologue.** "`take_ownership`: append `' - owned by Rust!'`
and return. So `s + ' - owned by Rust!'`? Hmm, `+` on `String` takes
`String` + `&str`. Try `s + ' - owned by Rust!'` — works. Test passes.
`borrow_string`: `.len()`. `mutate_string`: `s.push_str(...)`. All
green. ...is that it?"

**Wins.**
- The chapter is *deliberately mild on the surface*. The functions
  are tiny, the body is one line each. The whole point is to
  internalise the type signatures: `String` vs `&str` vs `&mut String`.
- The test for `take_ownership` has the comment
  `// Note: s is no longer valid here! It was moved.` which is the
  whole pedagogical payload. **Worth gold.**

**Sharp edges.**
- The exercises themselves are too easy for the topic's reputation.
  The learner who's read about ownership elsewhere will think "huh,
  that's it?" and miss the point. Three things would help:
  1. Add an "experiments" section to the intro: "After you pass the
     tests, try this: comment out the `result =` part and add
     `println!('{}', s);` after the call. Read the error."
  2. Add one extra (failing!) test inside `mod ignored_experiments`
     or similar that *intentionally* tries to use a moved value, so
     the learner sees the canonical error message.
  3. The `mutate_string` test passes a `&mut String`. Have them try
     to call it twice in a row with two separate `&mut` borrows and
     see what happens — actually that already works because the
     borrows don't overlap. Show them what *doesn't* work:
     `let r1 = &mut s; let r2 = &mut s; r1.push_str("x"); r2.push_str("y");`.
- `borrow_string(s: &str)` taking `&str`, but the test calls it with
  a `&'static str` literal — they don't see the difference between
  `&String` and `&str` until later. Could be deliberate (deref
  coercion does the right thing); worth one sentence in the prose.
- This chapter is rated **Spike** for state-of-mind reasons: the
  *concept* is hard even though the code is easy. The risk is they
  underestimate it and then panic in chapter 11 (closures + borrowing).

---

## 10 — Structs and methods  ·  *Moderate*

**State of mind.** Comfortable territory. Most languages have a
flavour of this.

**Inner monologue.** "`new`: `Self { email, name, is_verified: false,
login_count: 0 }`. Wait, can I use the field-init shorthand for
`email` and `name`? Yes. Nice. `display_name`: `format!('{} ({})',
self.name, self.email)`. `record_login`: `self.login_count += 1;
self.is_verified = true;`. `can_access_premium`: `self.is_verified
&& self.login_count >= 5`. Done."

**Wins.**
- Field-init shorthand is a quiet little joy.
- The progression `new` → `display_name` (`&self`) → `record_login`
  (`&mut self`) → `can_access_premium` (`&self`) maps cleanly onto
  ownership. They didn't even notice.

**Sharp edges.**
- `record_login` *always* sets `is_verified = true`, including on
  every subsequent login. The test only checks the first one, but
  some learners will read this as a bug ("shouldn't it set it once
  and leave it alone?"). The doc comment says "marks as verified
  after first login" which is a little ambiguous. Either tighten
  the spec ("sets it once after the first login") or add a test
  showing it's idempotent and intentional.
- The `User` struct has all-public-by-default fields (because of no
  `pub`), but they're in the same module so they're accessible
  anyway. The chapter on modules (14) will revisit this. Fine.
- No `Default` impl, no builder. The chapter is lean. Some advanced
  learners will *want* `#[derive(Default)]` and a `with_*` builder;
  the prose could mention these as "things that exist, you'll meet
  them later."

---

## 11 — Iterator patterns  ·  *Hard*

**State of mind.** This is the chapter that converts "I write Rust
loops" into "I write Rust." Mixed feelings: excitement at the
elegance, frustration at the borrow errors.

**Inner monologue.** "`calculate_total_revenue`: `sales.iter().sum()`.
Easy. `normalize_emails`:
`emails.into_iter().map(|e| e.to_lowercase()).collect()`. Easy.
`select_usernames_starting_with_a`:
`usernames.into_iter().filter(|u| u.starts_with('a')).collect()`...
hmm, type mismatch. `filter` gets `&&str`? Why two `&`? Oh because
`filter` always passes a reference to the element, and the element
itself was already `&str`. So `|u| u.starts_with('a')` should... wait,
it does work? Or does it not? Let me try `|&u| u.starts_with('a')`."

"`find_rust_files`: now the input is `&[&str]` so `.iter()` gives me
`&&str`. I need `.copied()` somewhere. Or `.cloned()`? What's the
difference?"

**Wins.**
- The four-exercise ladder is doing a lot of heavy lifting here.
  Each one is one method.
- Discovering `.cloned()` / `.copied()` is a genuine *"that's the
  Rust way to peel a reference layer"* moment.

**Sharp edges.**
- **The `&&str` in closures is the single biggest stumbling block in
  the whole course.** The intro file does mention it, but the actual
  type that comes back varies depending on `.iter()` vs `.into_iter()`.
  Concrete suggestion: add a "what does this iterator yield?" table
  to the prose:
  | Source | `.iter()` yields | `.into_iter()` yields |
  |---|---|---|
  | `Vec<T>` | `&T` | `T` |
  | `&[T]` | `&T` | `&T` |
  | `&Vec<T>` | `&T` | `&T` |
- `find_rust_files` has an *explicit lifetime annotation* (`<'file>`).
  This is the first time they see one. The intro doesn't prepare
  them for it. **Either remove the lifetime annotation (the compiler
  can elide it if you write `fn find_rust_files<'a>(files: &'a [&'a str]) -> Vec<&'a str>` — actually no, it can't here, it needs the binding)** or add a paragraph in the intro
  explaining what `<'file>` does.
  - Honestly the cleanest fix is to take `Vec<&'static str>` or just
    `Vec<String>` so the lifetime ceremony goes away. The lifetime
    isn't the lesson here; iterators are.
- `select_usernames_starting_with_a` taking `Vec<&str>` and returning
  `Vec<&str>` works because the `&str`s have the same (elided)
  lifetime. Fine.
- This is the chapter where `clippy` will start being aggressively
  helpful (or aggressively annoying). The hint to enable Format
  early is a good preparation.

---

## 12 — Password validator  ·  *Hard* (open-ended)

**State of mind.** Tired but proud — they've completed 11 chapters.
Now they get a "build something" exercise. The intro explicitly
celebrates this.

**Inner monologue.** "OK, `is_strong`: `self.score >= 70`. Do
`validate` next, only base requirements. Loop through `password.chars()`
and bump score for each criterion. Wait, do I check uppercase by
counting or just by `.any()`? `.any()` since each criterion is
binary. Build a `Vec<String>` of feedback as I go. Track strength.
Done. ...now `generate_secure_password`. The hint about
`SystemTime::nanos()` is helpful but I have no idea how to actually
*use* it as a seed."

**Wins.**
- The "suggested order" in the doc comment is brilliant. It turns
  an intimidating "build all these classes" exercise into five small
  clear steps.
- The "it's also fine to change all the code in this file" license
  unblocks experimenters.

**Sharp edges.**
- `generate_secure_password` is genuinely the hardest part of the
  whole course so far. The hint is a *direction* but not a worked
  pattern. Learners with weak prior programming experience will
  stall here for an hour. Two options:
  1. Provide a worked snippet in the intro markdown showing
     `nanos % charset.len()` as the index-picking trick. This isn't
     spoonfeeding the *exercise* (the exercise is wiring up the
     full validator); it's spoonfeeding *one technique*.
  2. Mark `generate_secure_password` as truly optional and have
     `test_password_generation` be `#[ignore]` by default with a
     comment explaining how to enable it.
- The `PasswordReport.feedback` is a `Vec<String>` and the test
  checks for substrings (`msg.contains("characters") ||
  msg.contains("length")`). A learner who writes
  `"Password too short"` instead of `"Add more characters"` fails
  the test for an unrelated reason. **Either loosen the test to
  match `len < 8` heuristically, or expand the substring set.**
- `PasswordAdvisor::suggest_improvements` mostly duplicates
  `feedback`. The doc says "actionable suggestions" but the test
  only asserts non-empty. Some learners will return `report.feedback.clone()`
  and pass. Honest: *that's a fine first solution* — the chapter
  could explicitly say "returning `report.feedback.clone()` passes
  the test; the goal is to think about what *more* you'd want."

---

## 13 — The `?` operator  ·  *Moderate*

**State of mind.** Glad to be back in tutorial-mode after the
open-ended chapter 12.

**Inner monologue.** "`add_parsed_numbers`: `let a = a.parse::<i32>()?;
let b = b.parse::<i32>()?; Ok(a + b)`. Beautiful. `count_file_lines`:
`fs::read_to_string(filename)?.lines().count()` — wait, that returns
`usize` but the function returns `Result<usize, ...>`. Need `Ok(...)`
around it. `sum_numbers_in_file`: this needs both error types. Use
`Box<dyn Error>`. `?` does the conversion automatically? Yes."

**Wins.**
- `?` collapsing five lines of `match` into one character is the
  best kind of payoff.
- `Box<dyn Error>` quietly absorbing two different error types via
  `?` is a "wait, that just *works*?" moment. Worth flagging in
  prose.

**Sharp edges.**
- The TODO-style comment inside `test_file_reading` is a great
  teaching device:
  ```
  // TODO: How can you fix the error in `assert_eq`?
  // "binary operation `==` cannot be applied to type `Result<usize, std::io::Error>`"
  ```
  But the test as written *will* pass once the implementation is
  correct. The TODO is misleading — it sounds like the *learner*
  has to fix the assertion, but actually they need to make their
  function return the right thing. Reword: *"This test pretends to
  be broken until your implementation returns `Ok(2)`. The error
  message in the comment is the one you'd see if you returned a
  bare `usize` instead of wrapping it in `Ok`."*
- The tests touch the filesystem with `fs::write("test.txt", ...)`
  and `fs::write("numbers.txt", ...)`. Two issues:
  1. **They're not `tempfile`-isolated.** Running tests from two
     different chapters concurrently or running `cargo test`
     repeatedly with stale state could fail. For a course this is
     OK, but worth documenting: *"Tests in this chapter create
     real files in the working directory and clean them up. If a
     test fails partway through, you may need to delete `test.txt`
     manually."*
  2. **Test order matters.** `cargo test` runs in parallel by
     default. If `test_file_reading` and `test_combined_errors`
     both write to different files it's fine, but the pattern is
     fragile. Worth a `#[serial]`-style note or a `tempfile`
     hint as a "what you'd do in real code".
- `Box<dyn Error>` is hand-wavy. The intro probably should say
  "this means 'any type that implements the `Error` trait, allocated
  on the heap'. Real projects often use the `anyhow` crate instead;
  this works without any dependencies."

---

## 14 — Modules and visibility  ·  *Easy* (but with a sharp landing)

**State of mind.** They expect a lecture-flavoured chapter. The
intro markdown is solid.

**Inner monologue.** "Make `add` `pub`. Make `Settings` `pub`. Make
`port` `pub`. Make `new` and `get_port` `pub`. Make `State`
`pub` and its variants `pub`. Done. Wait, the test for `get_status`
has `let state = ...; // Once you make State public, this will compile`
— no assertion?"

**Wins.**
- The "make this `pub`, then this, then this" cascade is a great
  way to internalise that `pub struct` doesn't make fields public.
  They'll learn this *by hitting the error*.

**Sharp edges.**
- **The `test_status` test has no assertion.** It just creates
  `state` and ends. As long as it compiles, it passes. A learner
  who just makes `State` public (without making the variants
  public) gets a *compile error*, not a test failure. That might
  be intentional but it's confusing — the test runner output will
  say "test_status passed" once it builds, even though the body is
  vacuous. Add `assert!(matches!(state, status::State::Running));`
  or similar so the test actually exercises something.
- The TODO comments say "fix the call" but the actual fix is on
  the *module side* (add `pub`), not the call site. Wording could
  be: *"Make these items public so the function below compiles."*
- `mod calculator { fn add ... }` makes `add` private; the
  `calculate(x, y)` wrapper is `fn calculate` (also private). It
  all works in this single-file crate, but the chapter would
  benefit from a sentence: *"Everything's private but works
  because tests live in the same module. Try splitting `calculator`
  into its own file (chapter intro shows how) and you'll see why
  `pub` matters across files."*

---

## 15 — Word counter  ·  *Moderate* (synthesis)

**State of mind.** "I've done all the building blocks. Now I'm
combining them." Confident, slightly self-conscious about whether
their combinations look "Rusty enough."

**Inner monologue.** "`count_words`:
`text.split_whitespace().map(|w| w.to_lowercase()).fold(HashMap::new(), |mut m, w| { *m.entry(w).or_insert(0) += 1; m })`.
Or with `for`. Whatever, both work. `most_common_word`:
`count_words(text).into_iter().max_by_key(|&(_, c)| c)`. Done.
`frequent_words`: filter by count, collect keys. `text_stats`:
total = `split_whitespace().count()`, unique = collected `HashSet`,
avg = total chars / total words as f64. Done."

**Wins.**
- `max_by_key` and `entry().or_insert()` clicking is the chapter's
  payoff.
- `text_stats` returning a `(usize, usize, f64)` is a callback to
  chapter 6's tuples. "Oh right, that pattern."

**Sharp edges.**
- `most_common_word` returning `Option<(String, usize)>` requires
  cloning the key out of the `HashMap` (or consuming the map with
  `into_iter`). Some learners will spend 20 minutes wrestling with
  borrow errors before figuring out `.into_iter()` consumes. Worth
  a hint: *"`into_iter()` on a `HashMap` gives you `(K, V)` pairs
  by value; that's what makes returning `(String, usize)` possible
  here."*
- `text_stats` with a `(usize, usize, f64)` return type is mildly
  awkward — a `struct TextStats` would be cleaner. The chapter
  intro could acknowledge: *"In real code you'd reach for a
  struct here; we're using a tuple to keep the focus on the
  iterator chain."*
- The "average word length" assertion uses
  `(avg_len - 4.33).abs() < 0.1` — float comparison done correctly,
  *but* without explanation. A one-line aside: *"Floats don't
  compare exactly; we check that the value is *close enough*."*

---

## 16 — Env parser  ·  *Moderate-to-Hard*

**State of mind.** "I'm parsing real files now." Feels professional.

**Inner monologue.** "`parse_env_line`: `split_once('=')`, validate
non-empty, trim, return `Ok((k, v))`. The intro covered
`split_once` so this is direct. `parse_env_file`: `for line in
content.lines()`, skip empty/comment, parse, insert. `get_env_var`:
`env.get(key)?.parse().ok()`. The intro showed exactly this
pattern. `validate_required_vars`: loop, check each, return on
first missing."

**Wins.**
- The intro markdown is unusually thorough for this chapter and it
  pays off. `split_once`, the generic function, raw strings — all
  introduced *before* they're needed.
- `get_env_var<T: FromStr>` is the first generic the learner
  *writes*. Big deal.

**Sharp edges.**
- `parse_env_line` test cases are subtly inconsistent with the doc
  comment. Doc says "no spaces around `=`" but the
  `parse_env_line("INVALID")` case (no `=` at all) is what fails,
  not a spacing issue. The test doesn't actually verify the "no
  spaces" rule. A learner who *does* trim spaces (the intro
  encourages it) will pass anyway. **Either drop the "no spaces"
  language or add a test like `parse_env_line("KEY = value")`
  with an explicit expected behaviour (probably "trim, accept").**
- `parse_env_file` returning `Result<HashMap, ParseError>` —
  what should it do when *one* line is malformed? Stop and return
  `Err`? Skip it? The function signature implies "stop", but real
  `.env` parsers are usually lenient. Worth a doc-comment line:
  *"Stop and return `Err` on the first malformed line — strict
  parsing is easier to debug."*
- `get_env_var<T: FromStr>` doesn't bound `T::Err`, which is fine
  here, but a learner who tries to `?` the parse instead of `.ok()`
  will hit a baffling error about `T::Err: From<ParseError>`. The
  intro should explicitly recommend `.ok()` for this exercise.
- **Order-of-operations gotcha:** `line.trim()` *then* check for
  `#`, not the other way around — a `   # comment` line is a
  valid comment and should be skipped. The intro shows the right
  order; just call it out.

---

## 17 — CSV parser  ·  *Hard*

**State of mind.** Last "real" chapter. They want to feel they've
graduated. The doc-comment "Good luck." doesn't help the nerves.

**Inner monologue.** "`parse_simple_csv_line`: `split(',').map(str::trim).collect()`.
Done. `parse_csv_line`: ...oh no. State machine. Inside-quote vs
outside-quote, doubled quotes mean a literal quote. OK, hand-rolled
loop. Push current field on `,` when not inside quotes, push
current char otherwise, toggle quote state on `"`, with the
escape-quote bit being two `"` in a row. Let me try.
...Test 1 passes. Test 2 with `"John ""Johnny"" Doe"`... fails.
I'm decoding doubled quotes wrong."

**Wins.**
- Solving `parse_csv_line` correctly is genuinely a milestone.
  *They built a real parser.*
- `parse_csv_file` and `csv_to_records` are easier than they look
  because of the iterator chain.

**Sharp edges.**
- **`parse_csv_line` is a difficulty cliff.** The tests jump from
  "split on comma" to "RFC 4180 quote escaping" with no
  intermediate. Suggestions:
  1. Provide a worked-out *pseudocode* in the intro (not the actual
     code). Something like:
     ```
     state = OUTSIDE_QUOTE
     for each char in line:
       if state == OUTSIDE_QUOTE:
         if char == '"': state = INSIDE_QUOTE
         elif char == ',': flush current field
         else: append to current field
       else: # INSIDE_QUOTE
         if char == '"': peek next; if also '"', append literal '"'; else state = OUTSIDE_QUOTE
         else: append to current field
     flush last field
     ```
     This isn't spoonfeeding the implementation — they still have
     to write it in Rust, deal with `chars().peekable()`, and
     handle the iterator borrowing. But it removes the "I have no
     idea how to even start" wall.
  2. Add a third intermediate test between the simple and the
     fully-quoted one: `r#"a,"b",c"#` → `["a", "b", "c"]`. Just
     basic quote-stripping, no embedded commas, no doubled quotes.
- `parse_csv_file` doesn't say what to do with empty input or a
  trailing newline. The test happens not to exercise these. Real
  CSVs often have a trailing newline; a learner who handles it
  unevenly will pass the test but ship a bug. *Worth a sentence in
  the intro.*
- `csv_to_records` zip-iteration is short but combines several
  things at once: `headers.iter().zip(row).map(...)` then
  `.collect::<HashMap<_, _>>()` then `.collect::<Vec<_>>()` for the
  outer. Worth one example in the intro showing
  `headers.iter().cloned().zip(row.iter().cloned()).collect::<HashMap<_, _>>()`.

---

## 18 — Quiz  ·  *Trivial* (by design)

**State of mind.** Reflective. They want to know whether all this
stuck.

**Sharp edges.**
- The exercise file itself is essentially empty. The whole point
  lives in the static HTML quiz. That's fine, but the page should
  make it *very* clear: there's no Rust code to write here, click
  the link.
- The current `fn main()` in the file ships a real binary just to
  print three lines. Could just be a doc-only file with no `main`.

---

# Cross-cutting observations

These came up across multiple chapters; flagging them here so they
don't have to be repeated in every entry.

1. **The `&&str` / `&&T` problem in closures** (chapters 11, 15,
   17). It's the single most-asked Rust beginner question. A
   dedicated short note in the cheatsheet ("inside `.filter`,
   `.find`, etc., you get `&Item`; if `Item` is itself a reference
   you get `&&T`, and `**x` or pattern-matching `|&x|` peels it")
   would help across the whole back half.

2. **The default Run/Format/Submit loop is great when tests pass,
   awkward when they don't.** Consider showing the failing test's
   `assert_eq!` line in the output panel, not just the panic
   message. Currently learners have to scroll the editor to find
   which assertion fired.

3. **There's no "give up and see the answer" button.** That's a
   deliberate pedagogical choice and probably the right one. But
   a "show a hint" button that reveals a pre-written paragraph (one
   per function) would close the gap between "stuck for 5 minutes"
   and "stuck for 45 minutes and now demoralised." The hints could
   live in a `2_hints.md` per chapter, mirroring `1_intro.md`.

4. **Tests run in parallel by default.** Chapters 13 (filesystem)
   and any future networking chapter are at risk. Either document
   this or thread `--test-threads=1` through `cargo test --example`
   somewhere in the harness.

5. **Pacing: chapters 0–6 are too easy, 9 is artificially easy,
   11 spikes hard, 12 is open-ended, 13 calms back down, 17 spikes
   again.** The shape is: easy plateau → spike → plateau → spike.
   That's actually a fine learning curve *if* the learner expects
   it. A short note on the dashboard ("chapters 11 and 17 are the
   two cliffs; budget extra time for them") would set expectations
   and reduce the "am I bad at this?" moment.

6. **The `todo!()` panic message is unfriendly to first-time
   runners.** Consider catching the panic in the run output and
   replacing `not yet implemented` with something like:
   *"Looks like this function still has `todo!()` in it. Replace
   that with your implementation and hit Run again."*

---

# Changelog — fixes applied from this audit

The items below have landed in the course repository. They are listed
in the order they appear in this document.

## Per-chapter

- **Chapter 1** — `test_tax_calculation` now also asserts
  `calculate_total_with_tax(100, 8.4) == 108`, which pins down
  rounding (the original two cases passed under either truncation or
  rounding). The doc-comment on `parse_positive_integer` now
  explicitly forward-references chapter 7/8: returning `0` on failure
  is a bad pattern, kept here only because `Result` hasn't been
  introduced yet.
- **Chapter 3** — `HttpStatus` now derives `Copy` (and `Clone`) and
  has a comment explaining why that's fine for variant-only enums, so
  calling two functions on the same value no longer trips the borrow
  checker.
- **Chapter 9** — added an *Experiments* section with three
  `#[test]` functions that have commented-out lines for the canonical
  ownership errors (use-after-move, two `&mut`, mixing `&` and
  `&mut`). The lesson is the error message; uncomment one at a time.
- **Chapter 11** — `find_rust_files` now returns `Vec<String>` and the
  explicit `<'file>` lifetime is gone; the focus stays on iterators.
  The chapter-level `//!` block now warns up front about `&&T` in
  closure arguments and links to the cheatsheet.
- **Chapter 12** — `test_feedback_quality` accepts any of `character`,
  `length`, `short`, `longer`, or `at least` (case-insensitive), and
  prints the actual feedback on failure so a learner who wrote
  "Password too short" no longer fails for the wrong reason.
- **Chapter 13** — the misleading `// TODO:` in `test_file_reading`
  is replaced with a one-line note explaining that the assertion is
  already correct; the work is in `count_file_lines`. A new prose
  section in `1_intro.md` warns about parallel tests writing to disk
  and shows `cargo test -- --test-threads=1`.
- **Chapter 14** — `test_status` now actually asserts
  `state == status::State::Running`, and the `State` enum derives
  `PartialEq` so the assertion compiles once the learner makes the
  variants public.
- **Chapter 17** — the `parse_csv_line` body now contains a
  three-step pseudocode skeleton, and a new `test_quoted_csv_basic`
  with `r#""a","b","c""#` sits *between* the simple-split case and
  the doubled-quote one.

## Cross-cutting

- **Cheatsheet** — added a *What does my closure receive?* table to
  the iterators section, covering the four common shapes
  (`Vec<i32>`, `Vec<String>`, `&[&str]`, `into_iter`). Plus a note
  that method calls auto-deref but `==`/`<`/`>` do not.
- **Hint system** — chapters can now ship a sibling note named
  `<n>_hints.md` whose slug is `hints`. The startup parser pulls it
  out of the regular notes list into a dedicated `Exercise.hints`
  field, and the exercise template renders it as a closed `<details>`
  disclosure between the intro and the editor ("Stuck? Show a hint —
  no spoilers, just a nudge"). Within each function's hint list, only
  the first item is shown initially; a *Show next hint (N left)*
  button reveals one more `<li>` per click, so a learner who only
  needs the first nudge isn't shown the answer. Hints written for
  chapters 1, 11, 12, and 17.
- **Failing-test output** — when a test fails, the test list now
  shows the trimmed assertion / panic snippet inline under the test
  name (the leading `thread '...' panicked at ...` header is
  stripped, capped at 6 lines). The full output remains one click
  away in *Compiler / runtime output*.
- **`todo!()` copy** — a panic with `not yet implemented` is
  rewritten to *“This function still has `todo!()` in it. Replace
  `todo!()` with your implementation, then run again.”* before being
  shown in the test list.
- **Pacing heads-up** — the dashboard now has a one-paragraph note
  above the exercise list flagging chapters 11 and 17 as the two
  intentional difficulty spikes and pointing to the per-chapter
  hints disclosure.
- **Title alignment** — several H1s lost their meta-suffixes
  ("Open-Ended Exercise", "Combining Concepts", "Complex Parsing
  Challenge"), and seven chapter directories were renamed so the
  on-disk slug matches the H1 the learner sees in the dashboard:
  `01_integer_handling` → `01_integer_operations`,
  `03_enums_and_matching` → `03_http_status_handling`,
  `04_vectors_basics` → `04_vector_basics`,
  `06_tuples` → `06_tuples_and_destructuring`,
  `10_structs_and_methods` → `10_user_account_management`,
  `14_modules` → `14_modules_and_visibility`,
  `16_env_parser` → `16_environment_file_parser`. SQL migration
  `004_rename_chapter_slugs.sql` rewrites `submissions.exercise_name`
  for the seven old keys so existing learner progress survives.
- **Concept-first titles (round 2)** — a follow-up pass on the same
  observation: titles should foreground the *Rust idea*, not the
  example domain. "HTTP Status Handling" became **Enums and Pattern
  Matching**, "User Account Management" reverted to **Structs and
  Methods**, "Data Processing with Iterators" became **Iterators**,
  and similar tightening for chapters 1, 2, 4, 5, 7, 8, 9. Eight
  more directories renamed to match (`01_integers`,
  `03_enums_and_pattern_matching`, `04_vectors`, `07_option`,
  `08_result`, `09_ownership_and_borrowing`, `10_structs_and_methods`,
  `11_iterators`). Migration `005_concept_first_chapter_slugs.sql`
  remaps `submissions.exercise_name`, chaining off both the original
  historical keys and the intermediate names from migration 004 so
  no learner progress is lost regardless of which migration state a
  database is currently in.

## Test naming

Test function names were inconsistent: some matched their target
(`test_count_chars`), others described a scenario (`test_email_validation`,
`test_quoted_csv`), and a few were ambiguous (`test_parsing`,
`test_settings`, `test_status`). Renamed every `#[test]` in `examples/`
to follow `test_<function_under_test>`, with a `_scenario` suffix where
one target function has multiple tests (chapters 12, 15, 17). The
`test_count_and_contains` test in chapter 4 was split into
`test_count_items` and `test_contains_item` so each test still covers a
single target. Chapter 9's exploration helpers keep their
`experiment_*` prefix because they exist to be uncommented, not to
assert behaviour. Chapter 18's vacuous quiz test became
`test_quiz_placeholder` so it still uses the `test_` prefix. Three
doc-comment references that named tests by their old names
(chapter 12's *Suggested order*, chapter 14's `get_port` hint) were
updated to point at the new names.

## Per-chapter (second pass)

A second sweep through the per-chapter sharp edges that hadn't yet
been addressed in code:

- **Chapter 0** — the doc comment on `format_welcome_message` now
  spells out that the function must *return* a `String`, calls out
  that `println!` is not the right macro here, and links
  [`format!`](https://doc.rust-lang.org/std/macro.format.html).
- **Chapter 4** — the `contains_item` doc comment no longer hints at
  "a one-call method on `Vec` that does this" (which would lead the
  learner straight into the `Vec::contains` / `&str` mismatch). It
  now shows the explicit `for` loop appropriate at this point in the
  course, plus a forward-pointer to the chapter-11 `iter().any(...)`
  one-liner.
- **Chapter 5** — `count_words` documents the `*map.entry(k).or_insert(0)
  += 1` idiom directly and explains why the naive
  `contains_key`/`get_mut`/`insert` approach fights the borrow checker.
- **Chapter 6** — `get_first_name` notes that the `(String, String)`
  parameter is moved (not `Copy`) and contrasts it with `swap_values`,
  which works on `(i32, i32)` precisely because tuples of `Copy` types
  are themselves `Copy`.
- **Chapter 7** — the chapter-level `//!` block now formally introduces
  closure syntax (`|args| body`) with two worked examples, and
  `find_user_by_id` ships a step-by-step type walk-through
  (`&(u32, String)` → `Option<&(u32, String)>` → `Option<&str>`) so
  the learner doesn't have to invent the `name.as_str()` jump on their
  own.
- **Chapter 8** — `validate_email` mentions lifetime elision explicitly
  ("the `&str` in the return type implicitly borrows from `email`"),
  `parse_percentage` is marked as the harder one rather than letting
  learners think they're stuck on a warmup, and the doc comment warns
  that `&'static str` errors can't hold `format!` output — switch to
  `String` if you want that.
- **Chapter 10** — `record_login`'s spec is now explicit that
  `is_verified = true` is intentionally idempotent, and
  `test_login_tracking` asserts the flag stays on after a second login.
- **Chapter 14** — the `// TODO:` comments on `calculate`,
  `create_settings`, and `get_status` are replaced with concrete
  guidance pointing at the *module side* ("make `add` `pub`", "make
  `Settings`, `new`, and `get_port` `pub`", "make `State` *and* its
  variants `pub`"). They also reinforce that `pub struct` doesn't
  publish fields or methods.
- **Chapter 15** — `most_common_word` documents the `into_iter`
  trick that makes returning an owned `(String, usize)` possible;
  `text_stats` notes that a 3-tuple is awkward in real code and a
  `struct TextStats` would be the better real-world choice; the
  float-tolerance assertion in `test_text_statistics` carries a short
  inline note explaining why direct `==` on `f64` is a bug magnet.
- **Chapter 16** — `parse_env_line` documents that whitespace around
  `=` is *trimmed*, not rejected, and a new assertion
  (`parse_env_line("KEY = value")`) pins that behaviour down. The
  numbered-step comment swaps to trim-then-validate so you don't reject
  `KEY = ` for the wrong reason. `parse_env_file` documents the
  trim-then-comment-check ordering and the "strict: stop on the first
  bad line" decision. `get_env_var` recommends `.ok()` and explains
  why `?` doesn't work without an extra `From<T::Err>` bound.
- **Chapter 17** — `parse_csv_file` points at `str::lines` and
  reassures the learner that a trailing newline is handled for free;
  `csv_to_records` ships a worked `headers.iter().cloned().zip(...)`
  sketch so the chained `.collect()`s aren't a guessing game.

# Multi-step split

In the migration to multi-step chapters, each chapter directory was
rebuilt around one `<N>_<slug>.rs` file per concept. `1_intro.md`
holds the chapter prose, `2_hints.md` holds the chapter-wide hints
(when present), and `main.rs` is generated by `build.rs` from the
step files. Recorded boundaries:

- **Chapter 00 — `hello_rust`.** Single code step `4_welcome.rs`
  (`format_welcome_message`). Order 4 because the chapter ships notes
  at orders 1, 2, 3 (`1_intro.md`, `2_getting_unstuck.md`,
  `3_concepts.md`).
- **Chapter 01 — `integers`.** Three steps at orders 3-5
  (`number_to_string`, `calculate_total_with_tax`,
  `parse_positive_integer`). Order 2 reserved for `2_hints.md`.
- **Chapter 02 — `strings_and_chars`.** Four steps: `2_count_chars`,
  `3_shout`, `4_has_uppercase`, `5_first_char`. The borrowed/owned
  table stays in `1_intro.md`; per-step intros focus on the one
  method/concept the step exercises.
- **Chapter 03 — `enums_and_pattern_matching`.** Two steps:
  `2_status_code`, `3_should_retry`. The `HttpStatus` enum (with the
  `Copy` rationale comment) is duplicated into both step files since
  step modules are compiled independently in the generated aggregator.
- **Chapter 04 — `vectors`.** Four steps: `2_count_items`,
  `3_add_item`, `4_contains_item`, `5_create_shopping_list`. Tests
  moved next to their respective functions.
- **Chapter 05 — `hashmaps`.** Four steps: `2_create_default_config`,
  `3_set_config_value`, `4_get_config_value`, `5_count_words`. Each
  step file carries its own `use std::collections::HashMap;`.
- **Chapter 06 — `tuples_and_destructuring`.** Four steps:
  `2_get_user_info`, `3_rectangle_measurements`, `4_get_first_name`,
  `5_swap_values`. The move-semantics note on `get_first_name` is
  preserved and remains the natural cliff before chapter 9.
- **Chapter 07 — `option`** *(pilot).* Four steps: `2_fallback`,
  `3_transform`, `4_first_item`, `5_find_user`. The chapter intro
  (with the closure note) lives in `1_intro.md`. This was the first
  chapter migrated and locked the convention.
- **Chapter 08 — `result`.** Four steps: `2_safe_divide` (first
  `Result` with `&'static str` errors), `3_read_config` (owning
  `String` in `Ok`), `4_validate_email` (borrowed `&str` in `Ok`,
  lifetime elision preview), `5_parse_percentage` (multi-stage parse
  with distinct errors).
- **Chapter 09 — `ownership_and_borrowing`** *(spike).* Four steps:
  `2_take_ownership` (move + mutation without `&mut`),
  `3_borrow_string` (`&str` shared borrow), `4_mutate_string`
  (`&mut String`), `5_experiments` (the three borrow-check
  trip-wires). The two earlier helpers are pre-filled inside step 5
  so the experiments stand alone.
- **Chapter 10 — `structs_and_methods`.** Four steps: `2_new`
  (associated `new` returning `Self`), `3_display_name` (`&self`
  borrow + `format!`), `4_record_login` (`&mut self` mutation,
  idempotent flag), `5_can_access_premium` (predicate over two
  fields). The `User` struct is duplicated in every step; each step's
  `impl` carries the constructor plus whatever earlier-step helpers
  the test needs.
- **Chapter 11 — `iterators`** *(hard).* Four steps starting at
  order 3 (since `2_hints.md` exists): `3_sum`
  (`calculate_total_revenue`, collapse via `sum`), `4_map`
  (`normalize_emails`), `5_filter`
  (`select_usernames_starting_with_a`, including the `&&T` gotcha),
  `6_filter_to_string` (`find_rust_files`, filter `&[&str]` and own
  with `to_string`).
- **Chapter 12 — `password_validator`** *(spike, open-ended).* Five
  steps starting at order 3 (since `2_hints.md` exists):
  `3_is_strong` (warm-up, introduces shared `PasswordReport` /
  `PasswordStrength`), `4_char_classes` (the four `has_*` predicates),
  `5_generate` (`PasswordGenerator::generate_secure_password`),
  `6_advisor` (`PasswordAdvisor::suggest_improvements`), `7_validate`
  (orchestrator that re-stubs the shared types and the four `has_*`
  helpers as `todo!()` bodies, so the step compiles standalone).
- **Chapter 13 — `question_mark_operator`.** Three steps:
  `2_add_parsed_numbers` (`?` with one error type),
  `3_count_file_lines` (`?` with `io::Error` — still hits the
  intentional `Result<_, io::Error>: !PartialEq` lesson when
  unsolved), `4_sum_numbers_in_file` (`?` across multiple error
  types via `Box<dyn Error>`).
- **Chapter 14 — `modules_and_visibility`.** Three steps:
  `2_calculate` (`pub fn`), `3_settings` (`pub struct` and that
  fields and methods stay private until individually `pub`),
  `4_status` (`pub enum` and its variants). Each step is intentionally
  a compile error in its starter form — that's the lesson.
- **Chapter 15 — `word_counter`.** Four steps: `2_count_words`
  (split + lowercase + `entry().or_insert()`), `3_most_common_word`
  (`into_iter().max_by_key`), `4_frequent_words` (filter-and-collect),
  `5_text_stats` (orchestrator returning `(usize, usize, f64)`).
  Steps 3-5 each re-stub `count_words` so they compile standalone.
- **Chapter 16 — `environment_file_parser`.** Four steps:
  `2_parse_line` (`parse_env_line` + `ParseError`),
  `3_parse_file` (`parse_env_file`, re-stubs the helper),
  `4_get_var` (generic `get_env_var<T: FromStr>`),
  `5_validate` (`validate_required_vars` via `Iterator::find`).
- **Chapter 17 — `csv_parser`** *(spike).* Four steps starting at
  order 3 (since `2_hints.md` exists): `3_simple_line`
  (`parse_simple_csv_line`), `4_quoted_line` (`parse_csv_line`,
  state machine with quote escapes), `5_parse_file`
  (`parse_csv_file`, re-stubs the previous helper),
  `6_records` (`csv_to_records`, zip headers with rows into
  `HashMap`).
- **Chapter 18 — `rust_fundamentals_quiz`.** Stays single-step. The
  whole "chapter" is just a pointer to `static/quiz.html`; there's
  nothing to split.

## Cross-cutting notes from the migration

- **Self-contained steps.** Every step file compiles on its own. When
  a later step needs a helper from an earlier step, the helper is
  re-stubbed (with `todo!()`) in the later step so the learner gets a
  clean test failure rather than a cascading compile error from an
  unsolved earlier step. The `//!` intro for those steps says so
  explicitly.
- **Shared types are duplicated.** Step files are independent modules
  in the `build.rs`-generated aggregator, so any enum/struct used by
  more than one step gets its definition copy-pasted into each step
  file. The duplication is annotated with the same comments as the
  original, so the learner reading any one step sees the full context.
- **Tests live with their function.** Each step ends with
  `#[cfg(test)] mod tests { use super::*; ... }` containing only the
  tests for that step's function. This makes
  `cargo test --example NN_chapter _<n>_<slug>::` filter exactly one
  step's tests.
- **Order numbers.** The leading number on a step file is purely for
  ordering — collisions with note files (`1_intro.md`, `2_hints.md`)
  are avoided by starting code steps at order 2, 3, or 4 depending on
  what notes the chapter ships with. The leading number ends up in
  the generated module name (`_2_unwrap`) but never in the visible
  slug.
