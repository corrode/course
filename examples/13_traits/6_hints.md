# Hints

## display_temperature

1. The whole `fmt` body is one `write!` call.
2. `write!` takes the formatter, then a format string, then the args:
   `write!(f, "{:.1}°C", self.celsius)`.
3. Don't forget the return: `write!` already returns
   `fmt::Result`, so its result *is* your return value. No semicolon
   on the last line, or use an explicit `return`.
4. ```rust
   impl fmt::Display for Temperature {
       fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
           write!(f, "{:.1}°C", self.celsius)
       }
   }
   ```

## describable

1. Two `describe` methods are one-line `format!` calls:
   - `format!("{} by {}", self.title, self.author)` for `Book`.
   - `format!("{} ({})", self.title, self.year)` for `Movie`.
2. For `print_descriptions`, build a `Vec<String>` and join it:
   ```rust
   let lines: Vec<String> = items.iter().map(|x| x.describe()).collect();
   lines.join("\n")
   ```
3. If you're not comfortable with `.map`/`.collect` yet (chapter 16
   covers them properly), a plain `for` loop works just as well:
   ```rust
   let mut lines: Vec<String> = Vec::new();
   for item in items {
       lines.push(item.describe());
   }
   lines.join("\n")
   ```

## logger

1. `PlainLogger::log` is one line: `msg.to_string()`. That's all.
   Do not write `warn` or `error` for `PlainLogger`; the defaults
   already do the right thing.
2. `TaggedLogger::log` is also one line: `format!("{}: {}", self.tag,
   msg)`. The default `warn` calls this `log`, so `warn("slow")`
   ends up as `"auth: [WARN] slow"` automatically.
3. `TaggedLogger::error` is the override. The pattern mirrors the
   default body, just with a different prefix:
   ```rust
   fn error(&self, msg: &str) -> String {
       self.log(&format!("[CRITICAL] {msg}"))
   }
   ```
4. Notice the symmetry with object-oriented "inheritance":
   `PlainLogger` inherits both defaults, `TaggedLogger` inherits one
   and overrides the other. The trait is the only place a default
   ever lives, so there's no surprise about where behavior comes
   from.

## validate

1. The two missing `check` impls follow `MinLength` exactly. Use
   `input.contains(&self.needle)` and the inverse:
   ```rust
   if !input.contains(&self.needle) {
       Err(format!("must contain '{}'", self.needle))
   } else {
       Ok(())
   }
   ```
   and:
   ```rust
   if input.contains(&self.forbidden) {
       Err(format!("must not contain '{}'", self.forbidden))
   } else {
       Ok(())
   }
   ```
2. For `collect_errors`, a plain `for` loop is the most readable:
   ```rust
   let mut errors = Vec::new();
   for v in validators {
       if let Err(msg) = v.check(input) {
           errors.push(msg);
       }
   }
   errors
   ```
3. The slice element type `&dyn Validator` means each `v` in the loop
   is a `&&dyn Validator`. Method calls auto-deref, so `v.check(input)`
   works without ceremony.
4. Once you've met iterators (chapter 16), the same body collapses
   to `validators.iter().filter_map(|v| v.check(input).err()).collect()`.
