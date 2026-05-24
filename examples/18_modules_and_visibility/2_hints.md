# Hints

## calculate

1. The compiler complains about a private item with a path like
   `calculator::add`. Look at where `add` is declared.
2. Functions inside a module are private by default. Add `pub` in
   front of the `fn` keyword on the one the compiler is naming.
3. ```rust
   mod calculator {
       pub fn add(a: i32, b: i32) -> i32 {
           a + b
       }
   }
   ```

## get_status

1. The compiler is calling `status::State` private. That's the
   whole error.
2. Make the enum itself `pub`. The variants come along for free
   (unlike struct fields).
3. ```rust
   mod status {
       #[derive(Debug, PartialEq)]
       pub enum State {
           Running,
           Stopped,
       }
   }
   ```

## create_settings

1. The first error is about the `Settings` *type* being private.
   `pub` it. Compile again. The next error is about `Settings::new`
   being private. `pub` it. Compile again. The next error is about
   `get_port` being private. You see where this is going.
2. `pub struct Settings` does not make the fields or methods
   public. Each item gets its own `pub`.
3. ```rust
   mod config {
       pub struct Settings {
           port: u32,         // still private; not used from outside
       }

       impl Settings {
           pub fn new(port: u32) -> Self {
               Settings { port }
           }

           pub fn get_port(&self) -> u32 {
               self.port
           }
       }
   }
   ```
