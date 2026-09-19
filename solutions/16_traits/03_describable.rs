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

#[test]
fn books_use_their_own_fields() {
    let mut book = Book {
        title: "Dune".into(),
        author: "Herbert".into(),
    };
    assert_eq!(Describable::describe(&book), "Dune by Herbert");
    book.title = "Hyperion".into();
    book.author = "Simmons".into();
    assert_eq!(Describable::describe(&book), "Hyperion by Simmons");
    assert_eq!(book.title, "Hyperion");
}

#[test]
fn movies_use_their_own_fields() {
    let mut movie = Movie {
        title: "Arrival".into(),
        year: 2016,
    };
    assert_eq!(Describable::describe(&movie), "Arrival (2016)");
    movie.title = "Spirited Away".into();
    movie.year = 2001;
    assert_eq!(Describable::describe(&movie), "Spirited Away (2001)");
    assert_eq!(movie.title, "Spirited Away");
}
