//! # Enums and Pattern Matching
//!
//! Enums are one of Rust's superpowers! 🚀
//! 
//! Think of enums as a way to say "a value is ONE of several possible things."
//! They're like a choose-your-own-adventure book - you can go down different
//! paths, but only one at a time.
//!
//! The `match` expression is Rust's pattern matching - it's like a super-powered
//! switch statement that the compiler guarantees is exhaustive (you handle ALL
//! cases). This prevents bugs that plague other languages!
//!
//! Real-world story: NASA's Mars Pathfinder mission used similar enum concepts
//! to handle different spacecraft states. Mission-critical software needs to
//! handle every possible scenario!

/// Authentication status for our app users
/// This replaces messy boolean combinations with clear, explicit states
#[derive(Debug, PartialEq)]
enum AuthStatus {
    Unauthenticated,
    User(String),        // Store username
    Admin(String),       // Store admin username
}

/// Password security levels
/// Better than arbitrary numbers - these have clear meaning!
#[derive(Debug, PartialEq)]
enum Security {
    Weak,
    Normal, 
    Strong,
}

/// HTTP response status codes (simplified)
/// Web servers use enums like this to represent different response types
#[derive(Debug, PartialEq)]
enum HttpStatus {
    Ok,
    NotFound,
    ServerError,
    Forbidden,
}

/// Check if a user can access admin features
/// Expected output: true only for AuthStatus::Admin(_), false for others
fn can_access_admin(auth: &AuthStatus) -> bool {
    todo!()
}

/// Get a user's display name
/// Expected output: "Guest" for Unauthenticated, username for User(_) and Admin(_)
fn get_display_name(auth: &AuthStatus) -> String {
    todo!()
}

/// Determine password security level based on length
/// Expected output: Weak (<8 chars), Normal (8-12 chars), Strong (13+ chars)
fn check_password_security(password: &str) -> Security {
    todo!()
}

/// Convert HTTP status to status code number
/// Expected output: Ok->200, NotFound->404, Forbidden->403, ServerError->500
fn status_to_code(status: &HttpStatus) -> u16 {
    todo!()
}

/// Get appropriate message for HTTP status
/// Expected output: Ok->"Success", NotFound->"Page not found", Forbidden->"Access denied", ServerError->"Internal server error"
fn status_message(status: &HttpStatus) -> &'static str {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_access() {
        assert_eq!(can_access_admin(&AuthStatus::Unauthenticated), false);
        assert_eq!(can_access_admin(&AuthStatus::User("alice".to_string())), false);
        assert_eq!(can_access_admin(&AuthStatus::Admin("root".to_string())), true);
    }

    #[test]
    fn test_display_names() {
        assert_eq!(get_display_name(&AuthStatus::Unauthenticated), "Guest");
        assert_eq!(get_display_name(&AuthStatus::User("alice".to_string())), "alice");
        assert_eq!(get_display_name(&AuthStatus::Admin("root".to_string())), "root");
    }

    #[test]
    fn test_password_security() {
        assert_eq!(check_password_security("weak"), Security::Weak);
        assert_eq!(check_password_security("password"), Security::Normal);
        assert_eq!(check_password_security("very_secure_password"), Security::Strong);
    }

    #[test]
    fn test_http_codes() {
        assert_eq!(status_to_code(&HttpStatus::Ok), 200);
        assert_eq!(status_to_code(&HttpStatus::NotFound), 404);
        assert_eq!(status_to_code(&HttpStatus::Forbidden), 403);
        assert_eq!(status_to_code(&HttpStatus::ServerError), 500);
    }

    #[test]
    fn test_status_messages() {
        assert_eq!(status_message(&HttpStatus::Ok), "Success");
        assert_eq!(status_message(&HttpStatus::NotFound), "Page not found");
        assert_eq!(status_message(&HttpStatus::Forbidden), "Access denied");
        assert_eq!(status_message(&HttpStatus::ServerError), "Internal server error");
    }

    // Bonus exercises - run with: cargo test --example 10_enums_and_matching --features bonus
    #[cfg(feature = "bonus")]
    mod bonus {
        use super::*;

        /// BONUS: File processing result with different error types
        /// Real file systems have many different failure modes!
        #[derive(Debug, PartialEq)]
        enum FileResult<T> {
            Success(T),
            NotFound,
            PermissionDenied,
            DiskFull,
        }

        /// BONUS: Implement a simple calculator with operations
        /// Functional programming languages love algebraic data types like this!
        #[derive(Debug, PartialEq)]
        enum MathOperation {
            Add(f64, f64),
            Subtract(f64, f64), 
            Multiply(f64, f64),
            Divide(f64, f64),
        }

        /// BONUS: Traffic light state machine
        /// Embedded systems use enums to model state machines safely!
        #[derive(Debug, PartialEq)]
        enum TrafficLight {
            Red,
            Yellow,
            Green,
        }

        /// BONUS: Calculate the result of a math operation
        fn calculate(op: &MathOperation) -> Result<f64, &'static str> {
            todo!()
        }

        /// BONUS: Get the next traffic light state
        fn next_light(current: &TrafficLight) -> TrafficLight {
            todo!()
        }

        /// BONUS: Handle file operation results generically
        fn handle_file_result<T>(result: &FileResult<T>) -> String 
        where
            T: std::fmt::Debug,
        {
            todo!()
        }

        #[test]
        fn test_math_operations() {
            assert_eq!(calculate(&MathOperation::Add(2.0, 3.0)), Ok(5.0));
            assert_eq!(calculate(&MathOperation::Divide(10.0, 2.0)), Ok(5.0));
            assert_eq!(calculate(&MathOperation::Divide(10.0, 0.0)), Err("Division by zero"));
        }

        #[test]
        fn test_traffic_lights() {
            assert_eq!(next_light(&TrafficLight::Red), TrafficLight::Green);
            assert_eq!(next_light(&TrafficLight::Green), TrafficLight::Yellow);
            assert_eq!(next_light(&TrafficLight::Yellow), TrafficLight::Red);
        }

        #[test]
        fn test_file_results() {
            let success: FileResult<String> = FileResult::Success("data".to_string());
            assert_eq!(handle_file_result(&success), "Success: \"data\"");
            assert_eq!(handle_file_result(&FileResult::<()>::NotFound), "Error: File not found");
        }
    }
}