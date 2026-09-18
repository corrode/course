/// A type that knows how to describe itself in one short line.
trait Describable {
    fn describe(&self) -> String;
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
}

#[derive(Debug)]
struct Movie {
    title: String,
    year: u16,
}

/// Implement `Describable` for `Book` so that
/// `Book { title: "Dune".into(), author: "Herbert".into() }.describe()`
/// returns `"Dune by Herbert"`.
impl Describable for Book {
    fn describe(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

/// Implement `Describable` for `Movie` so that
/// `Movie { title: "Arrival".into(), year: 2016 }.describe()`
/// returns `"Arrival (2016)"`.
impl Describable for Movie {
    fn describe(&self) -> String {
        format!("{} ({})", self.title, self.year)
    }
}

// One generic T cannot be both Book and Movie. An enum would work for a
// closed set of types; trait objects also accept future implementors.
// These references borrow the originals, so no Box or clone is needed.
/// Describe a mixed shelf in order, separated by newlines. Empty input returns "".
fn print_mixed_descriptions(items: &[&dyn Describable]) -> String {
    let mut lines = Vec::new();
    for item in items {
        lines.push(item.describe());
    }
    lines.join("\n")
}

#[test]
fn mixed_shelf_borrows_book_and_movie() {
    let book = Book {
        title: "Dune".into(),
        author: "Herbert".into(),
    };
    let movie = Movie {
        title: "Arrival".into(),
        year: 2016,
    };
    let shelf: Vec<&dyn Describable> = vec![&book, &movie];
    assert_eq!(
        print_mixed_descriptions(&shelf),
        "Dune by Herbert\nArrival (2016)"
    );
    assert_eq!(
        print_mixed_descriptions(&[&movie, &book]),
        "Arrival (2016)\nDune by Herbert"
    );
    assert_eq!(book.title, "Dune");
    assert_eq!(movie.year, 2016);
}

#[test]
fn mixed_shelf_can_be_empty() {
    assert_eq!(print_mixed_descriptions(&[]), "");
}
