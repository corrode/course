# Why this course is built the way it is

I built this course around a simple bet: Rust sinks in faster when you write code before you study every rule.
The choices below follow from that bet.

| A reading-first path | This course |
|---|---|
| Read several chapters before writing any code | You write code from the first exercise |
| Ownership dumped as theory early on | Ownership introduced as a spiral, consolidated late |
| Little "why should I care?" motivation | Problem-first: show the bug Rust prevents, then the fix |
| No feedback when your code is wrong | Tests and focused `todo!()` stubs that point at what's missing |

Here you learn Rust by hitting its rules in practice, not by reading about them in advance.
The borrow checker makes more sense after you've watched it reject something than it does as a list of rules up front, so we use that order throughout.

## Future directions

I haven't built all the ideas below, but I don't want to lose them:

- Difficulty signposting: an explicit "this chapter is a cliff" warning before the hard spots.
- A dedicated closures chapter (`Fn`, `FnMut`, `FnOnce`, and capture semantics) as another turn of the ownership spiral.
- A fearless-concurrency chapter, the fourth pillar alongside no null, no exceptions, and memory safety without a GC.
- A problem-first language picker: ask which language you're coming from and show the bug in that language first.
  For now, I keep the prose language-neutral.
- Pillar slogan titles for the chapters ("No null", "No exceptions", and so on).
  For now, I keep that motivation in the opening lines.
- A running project thread that carries one program across chapters instead of standalone exercises.

If you have opinions on any of these, the issue tracker is open.
