trait Describable {
    fn describe(&self) -> String;
}

// Write print_descriptions here, including its generic signature.
// Borrow a slice whose elements implement Describable.
// Return the descriptions in order with one newline between them.

#[cfg(test)]
mod tests {
    use super::*;

    struct Label(String);
    impl Describable for Label {
        fn describe(&self) -> String {
            self.0.clone()
        }
    }

    struct Number(i32);
    impl Describable for Number {
        fn describe(&self) -> String {
            self.0.to_string()
        }
    }

    #[test]
    fn borrows_and_preserves_order() {
        let labels = [Label("second".into()), Label("first".into())];
        assert_eq!(print_descriptions(&labels), "second\nfirst");
        assert_eq!(print_descriptions(&labels), "second\nfirst");
        assert_eq!(labels[0].0, "second");
    }

    #[test]
    fn accepts_another_implementor() {
        assert_eq!(print_descriptions(&[Number(42), Number(-7)]), "42\n-7");
    }

    #[test]
    fn empty_and_single_item_slices() {
        let empty: [Label; 0] = [];
        assert_eq!(print_descriptions(&empty), "");
        assert_eq!(print_descriptions(&[Number(9)]), "9");
    }

    #[test]
    fn empty_descriptions_still_have_separators() {
        let labels = [Label("".into()), Label("middle".into()), Label("".into())];
        assert_eq!(print_descriptions(&labels), "\nmiddle\n");
    }
}
