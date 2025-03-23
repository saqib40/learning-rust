// You can use existing struct instances to initialize a new struct using .. syntax.
struct Book {
    title: String,
    author: String,
    pages: u32,
}

pub fn elipsis() {
    let book1 = Book {
        title: String::from("The Rust Book"),
        author: String::from("Steve Klabnik"),
        pages: 500,
    };

    let book2 = Book {
        title: String::from("Advanced Rust"),
        ..book1 // copies the remaining fields.
    };

    println!("{} by {} ({} pages)", book2.title, book2.author, book2.pages);
}
