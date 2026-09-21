fn rectangle_measurements(width: u32, height: u32) -> (u32, u32) {
    todo!("Return the rectangle's area and perimeter as an (area, perimeter) tuple")
}

#[test]
fn test_rectangle_measurements() {
    let (area, perimeter) = rectangle_measurements(5, 3);
    assert_eq!(area, 15); // 5 * 3
    assert_eq!(perimeter, 16); // 2 * (5 + 3)
}
