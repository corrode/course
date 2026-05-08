//! # Rust Fundamentals Quiz
//!
//! Test your understanding of the key Rust concepts covered in this course!
//! This interactive quiz covers ownership, borrowing, types, pattern matching,
//! and other fundamental Rust principles.
//!
//! Click the link below to take the quiz in a new tab. The quiz includes
//! 20 questions with immediate feedback and explanations.

// This is a special quiz "exercise" - no actual Rust code is needed here.
// The quiz is an interactive HTML page that tests fundamental Rust concepts.

fn main() {
    println!("🎯 This is the Rust Fundamentals Quiz!");
    println!("📝 Test your knowledge of the concepts covered in this course");
    println!("🚀 Click the quiz link in your dashboard to get started");
}

#[cfg(test)]
mod tests {
    #[test]
    fn quiz_instructions() {
        // This test always passes - the quiz is completed externally
        assert!(true, "Quiz instructions are clear!");
    }
}