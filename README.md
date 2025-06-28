# Rust Workshop Exercises - Corrode

Learn Rust through practical, real-world exercises that you can actually use at work.

This is the official course repository for **corrode**, a friendly, professional Rust consultancy. 
The goal is to go from zero to building real applications in Rust as quickly as possible while having fun!

## What Makes This Course Different

Unlike other Rust learning resources, this course is **designed for the working developer** who wants to build real applications:

- **🎯 Work-Ready Skills**: Every exercise teaches patterns you'll use need for using Rust productively 
- **📈 Incremental Learning**: One concept per exercise, systematically writing more and more complex code 
- **🔗 Connected Concepts**: Exercises build on each other to reinforce learning
- **⚡ Quick Wins**: short exercises to maintain momentum
- **🛠️ Practical Examples**: User management, HTTP handling, config parsing, data processing: everything you typically do in your day job. (No inverse binary trees.)

**Compared to other resources:**
- **vs Rustlings**: Less theoretical, more practical workplace patterns, real-world context
- **vs Exercism**: Focus on production-ready applications, not just toy problems
- **vs The Rust Book**: More hands-on exercises, less theory, and more focus, more interactive learning
- **vs Rust by Example**: Less focus on going through the entire standard library, more focus on building real applications 
- **vs Other Exercise Collections**: cut straight to the chase with practical exercises

## Getting Started

### Prerequisites

- No prior Rust knowledge required
- Must have programming experience in Python, JavaScript, Go, or similar languages 
- Familiarity with basic programming concepts (variables, functions, loops, conditionals)

### Two Ways to Practice

#### Option 1: Rust Playground (Recommended for Beginners)

1. Open [play.rust-lang.org](https://play.rust-lang.org/)
2. Copy an exercise from `exercises/beginner/`
3. Implement the functions to make tests pass
4. Run tests with the "Test" button

#### Option 2: Local Development

Use your own development environment for a more realistic experience:

1. [Install Rust](https://rustup.rs/)
2. Clone this repository
3. Navigate to an exercise: `cd exercises/beginner`
4. Run tests: `cargo test --bin 00_hello_rust`

## Exercise Progression

### Beginner Track (14 exercises)

The exercises follow a carefully designed progression:

1. **00_hello_rust** - String creation and formatting
2. **01_integer_handling** - Arithmetic and number operations  
3. **02_enums_and_matching** - Pattern matching with HTTP status codes
4. **03_vectors_basics** - Growable arrays and basic operations
5. **04_hashmaps** - Key-value storage for configuration and caching
6. **05_tuples** - Multiple return values and destructuring
7. **06_option_handling** - Safe handling of missing values
8. **07_result_handling** - Error handling and validation
9. **08_ownership_basics** - Memory safety and borrowing
10. **09_structs_and_methods** - User account management
11. **10_iterator_patterns** - Data transformation with functional patterns
12. **11_word_counter** - **Combining concepts** (strings + vectors + hashmaps)
13. **12_env_parser** - **Practical parsing** (environment files)
14. **13_csv_parser** - **Complex parsing** (challenging final exercise)
15. **14_password_validator** - **Open-ended project** (be creative!)

Each exercise includes:
- Clear learning objectives
- Real-world context
- Unit tests
- Hints without hand-holding

## Learning Philosophy

### One Concept Per Exercise

Each exercise focuses on a single Rust concept to avoid cognitive overload.
You'll master each building block before moving to the next.

### Progressive Complexity

Early exercises establish fundamentals. Later exercises combine multiple
concepts to build realistic applications.

### Real-World Context

Every exercise solves problems you'll encounter in production:

- Error handling patterns
- Data processing pipelines
- Configuration management
- HTTP API development
- ...

### Practical Over Theoretical

We skip academic examples in favor of patterns you'll use at work.
Rust has a [famously steep learning curve](https://corrode.dev/blog/flattening-rusts-learning-curve/), so let's focus on skills that will help you be productive quickly.

## Tips for Success

1. **Read the module documentation** (`//!`) to understand the context
2. **Run tests frequently** to get immediate feedback
3. **Don't skip exercises** - each builds on the previous
4. **Experiment beyond the requirements** once tests pass
5. **Focus on understanding** over speed
6. **This is not a challenge**. Take your time!
7. **Ask for help** if you're stuck, we got your back! 

## Helpful Resources 

- **Rust Documentation**: [doc.rust-lang.org](https://doc.rust-lang.org/stable/std/index.html): use this as your primary reference. It's excellently written and contains everything you'll need.
- **The Rust Book**: [doc.rust-lang.org/book/](https://doc.rust-lang.org/book/): the official Rust book, great for understanding concepts in depth.
- **Rust by Example**: [doc.rust-lang.org/rust-by-example/](https://doc.rust-lang.org/rust-by-example/)

## About corrode

corrode is a Rust consultancy that helps teams adopt Rust for production applications.
We focus on practical training and real-world implementation patterns.
If you would like an in-person workshop, a remote training session, or a code review of your Rust code, please reach out to us.

Learn more at [corrode.dev](https://corrode.dev)

---

**Ready to start?** Open `exercises/beginner/00_hello_rust.rs` and begin your Rust journey! Good luck, and have fun!