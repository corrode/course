#![allow(dead_code, unused_variables)]

// A quick tour of Rust, on one page.
// Hover any keyword or type for a one-line explanation.
// You don't need to understand every detail yet. Just soak it in,
// then press Run to watch World 1-1 play out.

// `let` binds a name to a value. Bindings are immutable unless you add `mut`.
fn basics() {
    let player = "Mario"; // type inferred: a string slice (&str)
    let mut coins = 0; // `mut` lets this one change
    coins += 1;

    let lives: u8 = 3; // an explicit type: unsigned 8-bit integer
    let stats = (1, 1, 400); // a tuple groups values: (world, level, time)

    // `if` is an expression: it evaluates to a value you can bind.
    let status = if coins >= 100 {
        "1-Up!"
    } else {
        "keep collecting"
    };

    // Two kinds of loop.
    for enemy in ["Goomba", "Koopa"] {
        // runs once per item
    }
    let mut timer = 10;
    while timer > 0 {
        timer -= 1;
    }
}

// An `enum` is a type with a fixed set of variants. Variants can carry data.
enum PowerUp {
    Small,
    Mushroom,
    Star(u32), // this variant holds the seconds of invincibility left
}

// A `struct` groups related fields under one named type.
struct Player {
    name: String,
    power: PowerUp,
}

// A `trait` describes behavior that many types can share.
trait Jump {
    fn jump(&self);
}

// Implement the `Jump` behavior for our `Player`.
impl Jump for Player {
    fn jump(&self) {
        // `match` is exhaustive: every variant must be handled.
        match self.power {
            PowerUp::Star(secs) => println!("{} leaps, invincible for {secs}s!", self.name),
            PowerUp::Mushroom => println!("{} takes a high jump!", self.name),
            PowerUp::Small => println!("{} hops.", self.name),
        }
    }
}

// Ownership: every value has one owner. Assigning it hands ownership over.
fn ownership() {
    let shell = String::from("green shell");
    let kicked = shell; // `shell` is moved into `kicked`
    // println!("{shell}"); // would not compile: `shell` no longer owns the value
    println!("Mario kicks the {kicked}.");
}

// Borrowing: lend a value with `&` instead of giving it away.
fn add_points(score: &mut i32) {
    *score += 100; // `*` writes through the mutable reference
}
fn report(score: &i32) {
    println!("Score: {score}"); // a shared `&` reference is read-only
}

// `Option<T>` models a value that might be missing. Rust has no null.
fn hit_block(empty: bool) -> Option<&'static str> {
    if empty { None } else { Some("coin") }
}

// `Result<T, E>` models an operation that can fail. Rust has no exceptions.
fn enter_pipe(id: u8) -> Result<&'static str, &'static str> {
    match id {
        1 => Ok("warp zone"),
        _ => Err("piranha plant!"),
    }
}

// Every program starts at `main`.
fn main() {
    let mut mario = Player {
        name: String::from("Mario"),
        power: PowerUp::Mushroom,
    };

    println!("World 1-1");
    mario.jump();

    // `if let` runs the block only when the Option is `Some`.
    if let Some(item) = hit_block(false) {
        println!("Mario found a {item}!");
    }

    let mut score = 0;
    add_points(&mut score); // lend `score` mutably
    report(&score); // lend `score` just to read it

    // Grab a star and become invincible.
    mario.power = PowerUp::Star(10);
    mario.jump();

    // Handle both outcomes of a fallible call with `match`.
    match enter_pipe(1) {
        Ok(place) => println!("Mario enters the {place}."),
        Err(danger) => println!("Ouch: {danger}"),
    }

    println!("World 1-1 clear! Flagpole reached. 🏁");
}
