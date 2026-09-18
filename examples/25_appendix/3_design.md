# Why This Course Is Built the Way It Is

I built this course around a simple bet: Rust sinks in faster when you write code before you study every rule.
The choices below follow from that bet.

| A reading-first path | This course |
|---|---|
| Read several chapters before writing any code | You write code from the first exercise |
| Ownership dumped as theory early on | Ownership introduced as a spiral, consolidated late |
| Little "why should I care?" motivation | Problem-first: show the bug Rust prevents, then the fix |
| No feedback when your code is wrong | Tests and focused `todo!()` stubs that point at what's missing |

Here you try the code before reading the rules behind it.
I use that order because a borrow-checker error gives you something concrete to work through instead of a rule to memorize.

## Future Directions

I haven't built all the ideas below, but I don't want to lose them:

- More warning before the hard spots, so you know when to set aside extra time.
- A dedicated closures chapter (`Fn`, `FnMut`, `FnOnce`, and capture semantics) as another turn of the ownership spiral.
- A concurrency chapter on threads, shared state, and the compiler checks that prevent data races.
- A problem-first language picker: ask which language you're coming from and show the bug in that language first.
  For now, I keep the prose language-neutral.
- Pillar slogan titles for the chapters ("No null", "No exceptions", and so on).
  For now, I keep that motivation in the opening lines.
- A running project thread that carries one program across chapters instead of standalone exercises.

If you have opinions on any of these, the issue tracker is open.
