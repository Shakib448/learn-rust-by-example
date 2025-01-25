/*
Mutability

Mutable data can be mutably borrowed using &mut T. This is called a mutable reference and gives
read/write access to the borrower. In contrast, &T borrows the data via an immutable reference,
and the borrower can read the data but not modify it:

*/

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.author, book.title);
}

fn new_edition(book: &mut Book) {
    book.year = 2000;

    println!("I mutably borrowed {} - {} edition", book.author, book.title);
}

fn main() {
    let immutable = Book {
        author: "Douglas Hofstadter",
        title: "The title",
        year: 1979,
    };

    let mut mutabook = immutabook;
    borrow_book(&immutable);
    borrow_book(&mutabook);
    new_edition(&mut mutabook);
    new_edition(&mut immutable);
}