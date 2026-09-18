# Hints

## display_temperature

1. The whole `fmt` body is one `write!` call.
2. `write!` takes the formatter, then a format string, then the args.
3. Don't forget the return: `write!` already returns `fmt::Result`, so its result *is* your return value.
   No semicolon on the last line, or use an explicit `return`.

## describable

1. The two `describe` methods can use `format!`.
2. For `print_descriptions`, build a `Vec<String>` and join it with newlines.
3. If you're not comfortable with `.map`/`.collect` yet (the iterators chapter covers them properly), a plain `for` loop works just as well.

## logger

1. `PlainLogger::log` returns the message as an owned `String`.
   Do not write `warn` or `error` for `PlainLogger`; the defaults already do the right thing.
2. `TaggedLogger::log` adds the tag with `format!`.
   The default `warn` calls this `log`, so `warn("slow")` ends up as `"auth: [WARN] slow"` automatically.
3. `TaggedLogger::error` is the override.
   The pattern mirrors the default body, just with a `[CRITICAL]` prefix.
4. Notice the symmetry with object-oriented "inheritance": `PlainLogger` inherits both defaults, `TaggedLogger` inherits one and overrides the other.

## validate

1. The two missing `check` impls follow `MinLength` exactly.
   Use `str::contains` for the substring checks.
2. For `collect_errors`, use `if let` to distinguish failed checks from successful ones.
3. The slice element type `&dyn Validator` means each `v` in the loop is a `&&dyn Validator`.
   Method calls auto-deref, so `v.check(input)` works without an explicit dereference.
4. Once you've met iterators (the iterators chapter), `filter_map` and `Result::err` offer another way to collect failures.
