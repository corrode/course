/// Calculates both area and perimeter of a rectangle.
/// Returns (area, perimeter) as a tuple.
fn rectangle_measurements(width: u32, height: u32) -> (u32, u32) {
    let area = width * height;
    let perimeter = 2 * (width + height);
    (area, perimeter)
}

#[test]
fn test_rectangle_measurements() {
    let (area, perimeter) = rectangle_measurements(5, 3);
    assert_eq!(area, 15); // 5 * 3
    assert_eq!(perimeter, 16); // 2 * (5 + 3)
}
