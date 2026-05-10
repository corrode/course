# Hold on, another Rust course?

What's wrong with the [official Rust book](https://doc.rust-lang.org/book/)
and all those other great resources out there?
Nothing, really, and I hope that you'll still take a quick peek into those (before coming back).

To my defense, I was not planning to turn this into anything serious.
It sort of... just happened?

I personally had a hard time following the official Rust book when I was first learning the language a decade ago. It was a bit too much at once, and I had to have my editor open to play around with the examples. I wanted something more interactive, where I could run the code and see the results immediately. And I quick, structured path through the language, with exercises that build on each other; exercises, that were rooted in real-world problems, not just toy examples.

Throughout the years, I built up this catalog of exercises and examples that I pull out whenever someone's asking. I decided, that it's finally time to put it all together in one folder on my computer. And now it found its way on to your computer; isnt' that nice? 

# Why Rust?

[Last time I checked](https://hopl.info/home.prx), there were 8945 programming languages. That's a lot of programming languages. 
So, why not learn Racket or COBOL like the cool kids? 

A big one for me is that you can use Rust for literally anything from embedded programming all the way up to web development. And that's just super empowering. Another reason (for me) is that every Rust program teaches you something interesting about programming in general. Every method, every type in Rust is well thought out and has its place. Putting it all together, you get this beautiful thing, which is bigger than the sum of its parts. Beautiful Rust just snaps into place. It feels effortless.

There's beauty in systems programming. In some sense, it's simpler than higher level programming, because you have fewer abstractions between you and the machine, so there are fewer things that can go wrong. And that sense of control over the environment is quite joyful.

I stopped writing programs in other languages in any meaningful capacity. Rust was the first language where I felt home. I hope that you will feel the same way.

If you always wanted to know how computers really work, Rust is for you.


# Things to know before you start 

Rust is a Big Language with a "B."
There is no possible way I could teach you all of Rust in one sitting.
It's like one of these lifetime projects that you keep on chipping away at, and one day you look back and think "whoa." and that day is many years in the future.

It's tempting to take shortcuts.  
Don't take shortcuts!  
Shortcuts only mean that it takes you **longer** to get comfortable with Rust.

Learning Rust requires you to roll up your sleeves and get your hands dirty. You won't learn Rust by reading a book or watching videos. The code has to leave your fingers and transpire into the machine. The key to learning Rust is to write lots of Rust.

So, at any point in the book, take the time to think and research.
Every corner of computing is so rich and interesting, and Rust is the perfect language to explore it. But, also, you have to get the core mechanics down first. It's easy to get lost in the weeds, so stay pragmatic. Baby steps!

The exercises are deliberately tiny.
The first few focus on one core concept (like a particular type or pattern), but I promise that they will fit together nicely as we go along. The later exercises will build on the earlier ones, so it's worth going in order or, if you're familiar with some of the concepts, at least skim through the earlier exercises before you jump into the later ones. 

All exercises can be completed by using the **standard library only**, no external crates needed. The goal is to build fluency with Rust's core types and idioms, one small focused problem at a time.

And finally, few habits worth carrying through:

- **Read the doc comment first.** Each function's `///` block tells you what
  the tests expect. The function signatures are intentionally minimal, so
  you'll often want to look up which method on `&str`, `Vec<T>`, or
  `Option<T>` does what you need.
- **Run the tests early and often.** A failing test is a friendly nudge,
  not an obstacle. The compiler errors are often more useful than the test
  output itself.
- **It's fine to look things up.**
  [`std` docs](https://doc.rust-lang.org/std/) and
  [Rust by Example](https://doc.rust-lang.org/rust-by-example/) are your
  friends. Reaching for them is a normal part of writing Rust.

And now, without further ado, let's jump right in. 