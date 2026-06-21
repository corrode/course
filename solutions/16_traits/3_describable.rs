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

/// Generic function with a trait bound.
///
/// Accepts a slice of *any* type `T` that implements `Describable`,
/// calls `.describe()` on each element, and joins the results with a
/// newline (`"\n"`) between them. An empty slice returns `""`.
///
/// Note: because of the `T: Describable` bound, every element of one
/// call must be the same concrete type. Mixing `Book`s and `Movie`s
/// in the same call needs trait objects, which is the next step.
fn print_descriptions<T: Describable>(items: &[T]) -> String {
    let mut lines = Vec::new();
    for item in items {
        lines.push(item.describe());
    }
    lines.join("\n")
}

#[test]
fn test_book_describe() {
    let b = Book {
        title: "Dune".to_string(),
        author: "Herbert".to_string(),
    };
    assert_eq!(b.describe(), "Dune by Herbert");
}

#[test]
fn test_movie_describe() {
    let m = Movie {
        title: "Arrival".to_string(),
        year: 2016,
    };
    assert_eq!(m.describe(), "Arrival (2016)");
}

#[test]
fn test_print_descriptions_books() {
    let library = [
        Book {
            title: "Dune".to_string(),
            author: "Herbert".to_string(),
        },
        Book {
            title: "Hyperion".to_string(),
            author: "Simmons".to_string(),
        },
    ];
    assert_eq!(
        print_descriptions(&library),
        "Dune by Herbert\nHyperion by Simmons"
    );
}

#[test]
fn test_print_descriptions_movies() {
    let films = [Movie {
        title: "Arrival".to_string(),
        year: 2016,
    }];
    assert_eq!(print_descriptions(&films), "Arrival (2016)");
}

#[test]
fn test_print_descriptions_empty() {
    let films: [Movie; 0] = [];
    assert_eq!(print_descriptions(&films), "");
}
