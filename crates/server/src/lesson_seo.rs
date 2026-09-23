//! Editorial descriptions are independent of chapter directives and progress keys.
//! Unknown/new chapters use the catalog-derived fallback in `seo`.
pub fn description(slug: &str) -> Option<&'static str> {
    Some(match slug {
        "numbers_in_rust" => {
            "Practice Rust integers, floating-point numbers, arithmetic, and numeric types with interactive exercises and compiler feedback."
        }
        "strings_str_and_chars" => {
            "Understand String, &str, and char in Rust. Practice working with text, string slices, and Unicode in your browser."
        }
        "moves_and_copy" => {
            "Learn when Rust moves a value and when it copies one. Explore ownership transfers and the Copy trait through hands-on exercises."
        }
        "conditionals_and_loops" => {
            "Practice Rust if expressions, loop, while, and for. Learn how control flow and iteration work with runnable examples."
        }
        "functions" => {
            "Write Rust functions with parameters and return values. Practice expressions, statements, and reusable code in your browser."
        }
        "borrowing_and_ownership" => {
            "Learn Rust ownership and borrowing with shared and mutable references. Use compiler feedback to understand the borrow checker's rules."
        }
        "word_count_challenge" => {
            "Build a word counter in Rust. Combine strings, loops, and functions in a practical exercise with tests and hints."
        }
        "enums_and_pattern_matching" => {
            "Define Rust enums and handle their variants with match. Practice exhaustive pattern matching and modeling different kinds of values."
        }
        "vectors" => {
            "Store and process sequences with Rust Vec. Practice creating vectors, adding elements, indexing, and iterating over collections."
        }
        "hashmaps" => {
            "Learn Rust HashMap through interactive exercises. Store key-value pairs, look up values, and update entries safely."
        }
        "tuples_and_destructuring" => {
            "Group values with Rust tuples and unpack them with destructuring. Practice patterns that make structured data easier to work with."
        }
        "option_when_a_value_might_be_missing" => {
            "Handle missing values with Rust Option, Some, and None. Practice pattern matching and safe alternatives to unchecked unwrap calls."
        }
        "result_when_an_operation_might_fail" => {
            "Handle operations that can fail with Rust Result, Ok, and Err. Practice explicit error handling through interactive exercises."
        }
        "the_question_mark_operator" => {
            "Use Rust's question mark operator to propagate errors. Practice writing clearer functions that return Result without nested matches."
        }
        "structs_and_methods" => {
            "Model data with Rust structs and add behavior with impl blocks and methods. Practice building and using your own types."
        }
        "a_primer_on_lifetimes" => {
            "Understand Rust lifetimes and how references relate to the data they borrow. Practice lifetime annotations with compiler feedback."
        }
        "traits" => {
            "Define shared behavior with Rust traits and implement it for your types. Practice trait-based abstractions in your browser."
        }
        "iterators" => {
            "Transform and process Rust collections with iterators. Practice map, filter, collect, and iterator chains with interactive exercises."
        }
        "word_frequencies" => {
            "Build a Rust word-frequency counter. Combine strings, hash maps, and iterators in a practical project with tests and hints."
        }
        "password_validator" => {
            "Design a password validator in Rust in this optional project. Combine the course's concepts and experiment with your own validation rules."
        }
        "modules_and_visibility" => {
            "Organize Rust code with modules, use declarations, and visibility rules. Practice defining clear boundaries between parts of a program."
        }
        "parsing_structured_text_and_generics" => {
            "Parse structured text in Rust and explore generics. Practice converting input into typed data and handling parsing errors."
        }
        "state_machines_and_stateful_parsing" => {
            "Model a Rust parser as a state machine. Use enums and pattern matching to track state while processing structured input."
        }
        "even_more_csv_parsing" => {
            "Continue practicing CSV parsing in Rust. Work through structured-text parsing exercises and learn to handle more complex input."
        }
        "smart_pointers" => {
            "Explore Rust Box, Rc, and RefCell in this optional chapter. Learn how smart pointers support heap allocation and shared ownership."
        }
        "rust_fundamentals_quiz" => {
            "Check your understanding of Rust fundamentals with an interactive quiz. Review core concepts and get feedback on each answer."
        }
        "appendix" => {
            "Find next steps after the Rust course, optional projects to try, and background on the course and its author."
        }
        _ => return None,
    })
}
