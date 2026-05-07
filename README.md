# Rust Workshop Exercises - Corrode

![Screenshot of the course page](/static/assets/screenshot.jpg)

Learn Rust through practical, real-world exercises that you can actually use at work.

This is the official course repository for **corrode**, a friendly, professional Rust consultancy. 
The goal is to go from zero to building real applications in Rust as quickly as possible while having fun!

## What Makes This Course Different

Unlike other Rust learning resources, this course is **designed for the working developer** who wants to build real applications:

- **🎯 Work-Ready Skills**: Every exercise teaches patterns you'll use need for using Rust productively 
- **👷‍♀️ Project-Based**: Learn by working through tiny, useful projects
- **📈 Incremental Learning**: One concept per exercise, systematically writing more and more complex code
- **🧪 Learn from first principles**: Teaches you how to navigate the documentation to stop guessing and start learning
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

## corrode Course CLI

The repository includes a CLI tool for submitting exercises and tracking your progress. The CLI connects to a course server to:

- Register your participation
- Submit completed exercises
- Track your progress and completion status
- Run tests automatically and provide feedback
- Support pedantic submissions with formatting and linting to earn ⭐ stars

### CLI Installation

```bash
# Clone the repository
git clone https://github.com/corrode/course.git
cd course

# Install the CLI tool
cargo install --path .

# Initialize and register
cargo course init

# Submit an exercise
cargo course submit examples/00_hello_rust.rs

# Submit with pedantic formatting/linting to earn stars
cargo course submit examples/00_hello_rust.rs --pedantic

# Check your progress
cargo course status
```

**Important Notes**:
- The CLI requires a running course server. See the server setup section below, or use manual testing for offline practice.
- After completing an exercise, you can submit again with `--pedantic` to earn a ⭐ star by passing additional formatting and linting checks!

### Three Ways to Practice

#### Option 1: Local Development with CLI Tool (Recommended)

Use the corrode course CLI for the full experience including progress tracking and star earning:

1. [Install Rust](https://rustup.rs/)
2. Clone this repository: `git clone https://github.com/corrode/course.git`
3. Install the CLI tool: `cargo install --path .`
4. Initialize your progress: `cargo course init`
5. Submit exercises: `cargo course submit examples/00_hello_rust.rs`
6. Earn stars with pedantic submissions: `cargo course submit examples/00_hello_rust.rs --pedantic`
7. Check your progress: `cargo course status`

#### Option 2: Manual Testing

For basic testing without progress tracking:

1. [Install Rust](https://rustup.rs/)
2. Clone this repository: `git clone https://github.com/corrode/course.git`
3. Navigate to an exercise: `cd course`
4. Run tests: `cargo test --example 00_hello_rust`
5. Run bonus exercises: `cargo test --example 00_hello_rust --features bonus`

#### Option 3: Rust Playground (Beginners)

For trying exercises without installing anything:

1. Open [play.rust-lang.org](https://play.rust-lang.org/)
2. Copy an exercise from `examples/`
3. Implement the functions to make tests pass
4. Run tests with the "Test" button

## Server Setup (For Instructors)

The course includes a built-in server for progress tracking and administration:

### Quick Start

```bash
# Copy and configure environment
cp .env.example .env
# Edit .env with your admin token

# Start the server
cargo run --bin server

# Server will be available at:
# - http://localhost:3000 (landing page)
# - http://localhost:3000/admin?token=<your_token> (admin dashboard)
```

### Environment Configuration

Edit `.env` with your settings:

```env
# Required: Secret token for admin access
CORRODE_ADMIN_TOKEN=your_secret_admin_token_here

# Optional: Database location (defaults to ./playground.db)
DATABASE_URL=sqlite:./playground.db

# Optional: Server port (defaults to 3000)
PORT=3000
```

### Admin Features

- **Progress Dashboard**: View all participants and their completion status
- **Recent Submissions**: See latest submissions with expandable source code
- **Individual Progress**: Direct links to participant dashboards

## Exercise Progression

### Beginner Track (19 exercises)

The exercises follow a carefully designed progression from basic concepts to advanced projects:

#### Foundation (0-4): Data Types and Basics
0. **00_hello_rust** - Welcome to Rust! String basics and your first program
1. **01_basic_types** - Core data types: integers, floats, booleans, and chars  
2. **02_numbers_and_math** - Mathematical operations and type conversions
3. **03_strings_and_text** - Text processing, validation, and manipulation
4. **04_collections_basics** - Working with Vec<T> for data storage

#### Core Concepts (5-9): Rust's Type System  
5. **05_tuples_and_structs** - Multiple return values and destructuring
6. **06_option_and_some_none** - Safe handling of missing values with Option<T>
7. **07_result_and_error_handling** - Error handling and validation with Result<T, E>
8. **08_ownership_and_borrowing** - Memory safety and Rust's ownership system
9. **09_structs_and_methods** - User account management with custom types

#### Advanced Patterns (10-12): Real Programming
10. **10_enums_and_matching** - Pattern matching and algebraic data types
11. **11_iterators_and_loops** - Data transformation with functional patterns
12. **12_password_validator** - **Open-ended project** (be creative!)

#### Integration (13-18): Putting It All Together
13. **13_question_mark_operator** - **Error handling patterns** (? operator)
14. **14_word_counter** - **Combining concepts** (strings + vectors + hashmaps)
15. **15_modules** - **Code organization** (mod system and visibility)
16. **16_quiz** - **Knowledge check** (interactive fundamentals quiz)
17. **17_env_parser** - **Practical parsing** (environment files)
18. **18_csv_parser** - **Complex parsing** (challenging final exercise)

Each exercise includes:
- Clear learning objectives
- Real-world context
- Unit tests
- Hints without hand-holding
- **Bonus challenges** for curious learners (see below)

### Bonus Exercises: Go Deeper!

Most exercises include **bonus challenges** that let curious learners explore advanced concepts without blocking normal progress. Bonus exercises are:

- **Optional**: Skip them entirely if you prefer to focus on the main progression
- **Advanced**: Cover more complex patterns and real-world scenarios  
- **Practical**: Demonstrate techniques used in production systems
- **Feature-gated**: Disabled by default to avoid overwhelming beginners

#### Running Bonus Exercises

```bash
# Run normal tests (basic exercises only)
cargo test --example 01_basic_types

# Run bonus exercises too
cargo test --example 01_basic_types --features bonus

# Submit with CLI (normal exercises)
cargo course submit examples/01_basic_types.rs

# Submit bonus exercises with CLI
cargo course submit examples/01_basic_types.rs --features bonus
```

#### Why Bonus Exercises?

- **Flatter Learning Curve**: Beginners aren't overwhelmed with too many concepts at once
- **Advanced Practice**: Experienced developers can dive deeper without being bored
- **Real-World Relevance**: Bonus exercises often mirror complex patterns from production systems
- **Self-Paced Learning**: Learn at your own speed and according to your curiosity level

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

**Ready to start?** Open `examples/00_hello_rust.rs` and begin your Rust journey! Good luck, and have fun!