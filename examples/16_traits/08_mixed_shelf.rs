/// A type that knows how to describe itself in one short line.
trait Describable {
    fn describe(&self) -> String;
}

struct Book {
    title: String,
    author: String,
}

struct Movie {
    title: String,
    year: u16,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Describable for Movie {
    fn describe(&self) -> String {
        format!("{} ({})", self.title, self.year)
    }
}

fn print_mixed_descriptions(items: &[&dyn Describable]) -> String {
    todo!(
        "Describe the mixed shelf in order, separated by newlines; return an empty string for empty input"
    )
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

#[test]
fn accepts_other_implementors() {
    struct Poster;
    impl Describable for Poster {
        fn describe(&self) -> String {
            "Art print".into()
        }
    }
    assert_eq!(print_mixed_descriptions(&[&Poster]), "Art print");
}
