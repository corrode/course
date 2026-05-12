/// Swaps two values using tuple destructuring.
fn swap_values(pair: (i32, i32)) -> (i32, i32) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_values() {
        assert_eq!(swap_values((1, 2)), (2, 1));
        assert_eq!(swap_values((42, 100)), (100, 42));
    }
}
