pub struct Library {
    books: Vec<Book>,
}

pub struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    pub fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    pub fn new() -> Library {
        Library { books: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.books.len()
    }

    pub fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }

    pub fn print_books(&self) {
        for book in &self.books {
            println!("{}", book);
        }
    }

    pub fn oldest_book(&self) -> Option<&Book> {
        self.books.iter().min_by_key(|book| book.year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_len() {
        let mut library = Library::new();
        assert_eq!(library.len(), 0);
        assert!(library.is_empty());

        library.add_book(Book::new("Lord of the Rings", 1954));
        library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
        assert_eq!(library.len(), 2);
        assert!(!library.is_empty());
    }

    #[test]
    fn test_library_is_empty() {
        let mut library = Library::new();
        assert!(library.is_empty());

        library.add_book(Book::new("Lord of the Rings", 1954));
        assert!(!library.is_empty());
    }

    #[test]
    fn test_library_print_books() {
        let mut library = Library::new();
        library.add_book(Book::new("Lord of the Rings", 1954));
        library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
        // We could try and capture stdout, but let us just call the
        // method to start with.
        library.print_books();
    }

    #[test]
    fn test_library_oldest_book() {
        let mut library = Library::new();
        assert!(library.oldest_book().is_none());

        library.add_book(Book::new("Lord of the Rings", 1954));
        assert_eq!(
            library.oldest_book().map(|b| b.title.as_str()),
            Some("Lord of the Rings")
        );

        library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
        assert_eq!(
            library.oldest_book().map(|b| b.title.as_str()),
            Some("Alice's Adventures in Wonderland")
        );
    }
}
