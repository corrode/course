//! # Numbers and Mathematical Operations
//!
//! Time to get our hands dirty with some real number crunching! 🔢
//!
//! Did you know that NASA's Mars Climate Orbiter crashed in 1999 because 
//! of a units conversion error? One team used metric units, another used 
//! imperial units. The $327 million spacecraft was lost! [^mars] This is why 
//! careful handling of numbers and units is so important.
//!
//! Rust helps prevent many numeric errors at compile time. Let's explore 
//! how to work with numbers safely and effectively!
//!
//! [^mars]: See https://en.wikipedia.org/wiki/Mars_Climate_Orbiter#Cause_of_failure
//!
//!## Bonus Exercises
//!
//! This lesson contains bonus exercises for curious learners.
//! These are optional and can be skipped if you're short on time.
//! You can run the bonus tests in this lesson with:
//!
//! ```sh
//! cargo test --example 02_numbers_and_math --features bonus
//! ```

/// Calculate the total bill including tip
/// Restaurant POS systems use this calculation 
/// Expected output: bill_amount + (bill_amount * tip_percentage / 100)
/// Example: bill=100.0, tip=20.0 -> 120.0
fn calculate_bill_with_tip(bill_amount: f32, tip_percentage: f32) -> f32 {
    todo!()
}

/// Convert temperature from Celsius to Fahrenheit
/// Weather apps use this conversion constantly
/// Expected output: celsius * 9.0/5.0 + 32.0
/// Example: 0°C -> 32°F, 100°C -> 212°F
fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    todo!()
}

/// Calculate how many full pizzas we need for a party
/// Catering software needs this kind of "round up" math
/// Expected output: round UP the total slices needed divided by slices per pizza
/// Example: 10 people, 3 slices each, 8 slices per pizza -> need 4 pizzas (30 slices total)
fn pizzas_needed(people: u32, slices_per_person: u32, slices_per_pizza: u32) -> u32 {
    todo!()
}

/// Calculate the area of a circle given its radius
/// Graphics software and CAD applications use this constantly
/// Expected output: π * radius²
/// Example: radius=1.0 -> π (≈3.14159), radius=2.0 -> 4π (≈12.566)
/// See: https://doc.rust-lang.org/std/f32/consts/index.html for PI
fn circle_area(radius: f32) -> f32 {
    todo!()
}

/// BONUS: Calculate compound interest for a single year
/// Banking software uses this for savings calculations
/// Formula: final_amount = principal * (1 + rate)
/// See: https://doc.rust-lang.org/std/primitive.f64.html
fn simple_compound_interest(principal: f64, annual_rate: f64) -> f64 {
    todo!()
}

/// BONUS: Calculate the hypotenuse of a right triangle
/// Computer graphics and navigation systems use this constantly
/// Formula: c² = a² + b², so c = sqrt(a² + b²)
/// See: https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt
fn hypotenuse(a: f64, b: f64) -> f64 {
    todo!()
}

/// BONUS: Convert miles to kilometers
/// GPS and mapping applications use this conversion
/// 1 mile = 1.60934 kilometers
fn miles_to_kilometers(miles: f64) -> f64 {
    todo!()
}

/// BONUS: Calculate Body Mass Index (BMI)
/// Health applications calculate this from height and weight
/// Formula: BMI = weight(kg) / height(m)²
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bill_calculation() {
        assert_eq!(calculate_bill_with_tip(100.0, 20.0), 120.0);
        assert_eq!(calculate_bill_with_tip(50.0, 15.0), 57.5);
        assert_eq!(calculate_bill_with_tip(25.0, 0.0), 25.0);
    }

    #[test]
    fn test_temperature_conversion() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0); // The magical equal point!
    }

    #[test]
    fn test_pizza_calculation() {
        assert_eq!(pizzas_needed(10, 3, 8), 4);  // 30 slices needed, 32 slices in 4 pizzas
        assert_eq!(pizzas_needed(5, 2, 8), 2);   // 10 slices needed, 16 slices in 2 pizzas
        assert_eq!(pizzas_needed(1, 1, 8), 1);   // 1 slice needed, 8 slices in 1 pizza
    }

    #[test]
    fn test_circle_area() {
        let pi = std::f32::consts::PI;
        assert!((circle_area(1.0) - pi).abs() < 0.01);
        assert!((circle_area(2.0) - (4.0 * pi)).abs() < 0.01);
        assert_eq!(circle_area(0.0), 0.0);
    }
}

#[cfg(all(test, feature = "bonus"))]
mod bonus {
    use super::*;

    #[test]
    fn test_compound_interest() {
        // $1000 at 5% should be $1050
        assert!((simple_compound_interest(1000.0, 0.05) - 1050.0).abs() < 0.01);
        assert!((simple_compound_interest(500.0, 0.10) - 550.0).abs() < 0.01);
    }

    #[test]
    fn test_hypotenuse() {
        // 3-4-5 triangle
        assert!((hypotenuse(3.0, 4.0) - 5.0).abs() < 0.01);
        // 5-12-13 triangle
        assert!((hypotenuse(5.0, 12.0) - 13.0).abs() < 0.01);
    }

    #[test]
    fn test_distance_conversion() {
        assert!((miles_to_kilometers(1.0) - 1.60934).abs() < 0.01);
        assert!((miles_to_kilometers(5.0) - 8.0467).abs() < 0.01);
    }

    #[test]
    fn test_bmi_calculation() {
        // 70kg, 1.75m height should be about 22.86
        assert!((calculate_bmi(70.0, 1.75) - 22.86).abs() < 0.01);
        assert!((calculate_bmi(80.0, 1.80) - 24.69).abs() < 0.01);
    }
}