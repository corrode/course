# Making a function `pub`

Modules organise code and control what's accessible. By default,
everything inside a module is **private** to that module. The `pub`
keyword lifts the gate item-by-item.

This step is deliberately broken. Run `cargo check --example
14_modules_and_visibility` (or just hit Run in the editor) and read
the compiler error: it's telling you exactly which item needs to be
made `pub`. Fix the smallest thing that makes it compile.
