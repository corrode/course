# Rust Workshop Exercises - Corrode

Learn Rust through practical, real-world exercises that you can actually use at work.

This is the official course repository for **Corrode**, a Rust consultancy focused on teaching developers practical skills for production applications.

## What Makes This Course Different

Unlike other Rust learning resources, this course is designed for **working developers** who want to build real applications:

- **🎯 Work-Ready Skills**: Every exercise teaches patterns you'll use in production code
- **📈 Incremental Learning**: One concept per exercise, building systematically 
- **🔗 Connected Concepts**: Exercises build on each other to reinforce learning
- **⚡ Quick Wins**: 5-10 minutes per exercise to maintain momentum
- **🛠️ Practical Examples**: User management, HTTP handling, config parsing, data processing

**Compared to other resources:**
- **vs Rustlings**: Less theoretical, more practical workplace patterns
- **vs Rust Playground**: Structured progression with real-world context
- **vs Other Exercise Collections**: Focus on production-ready patterns, not toy problems

## Getting Started

### Prerequisites
- Basic programming experience (JavaScript, Python, or similar)
- No prior Rust knowledge required

### Two Ways to Practice

#### Option 1: Rust Playground (Recommended for Beginners)
1. Open [play.rust-lang.org](https://play.rust-lang.org/)
2. Copy an exercise from `exercises/beginner/`
3. Implement the functions to make tests pass
4. Run tests with the "Test" button

#### Option 2: Local Development
1. [Install Rust](https://rustup.rs/)
2. Clone this repository
3. Navigate to an exercise: `cd exercises/beginner`
4. Run tests: `cargo test --bin 00_string_formatting`

## Exercise Progression

### Beginner Track (14 exercises)

The exercises follow a carefully designed progression:

1. **00_string_formatting** - String creation and formatting
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
- Comprehensive tests
- Hints without hand-holding

## Learning Philosophy

### One Concept Per Exercise
Each exercise focuses on a single Rust concept to avoid cognitive overload. You'll master each building block before moving to the next.

### Progressive Complexity
Early exercises establish fundamentals. Later exercises combine multiple concepts to build realistic applications.

### Real-World Context
Every exercise solves problems you'll encounter in production:
- User authentication systems
- Configuration management
- Data processing pipelines
- HTTP API development
- Error handling patterns

### Practical Over Theoretical
We skip academic examples in favor of patterns you'll use at work. Learn Rust by building things that matter.

## Tips for Success

1. **Read the module documentation** (`//!`) to understand the context
2. **Run tests frequently** to get immediate feedback
3. **Don't skip exercises** - each builds on the previous
4. **Experiment beyond the requirements** once tests pass
5. **Focus on understanding** over speed

## Getting Help

- **Rust Documentation**: [doc.rust-lang.org](https://doc.rust-lang.org/)
- **The Rust Book**: [doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
- **Rust by Example**: [doc.rust-lang.org/rust-by-example/](https://doc.rust-lang.org/rust-by-example/)

## About Corrode

Corrode is a Rust consultancy that helps teams adopt Rust for production applications. We focus on practical training and real-world implementation patterns.

Learn more at [corrode.dev](https://corrode.dev)

---

**Ready to start?** Open `exercises/beginner/00_string_formatting.rs` and begin your Rust journey!