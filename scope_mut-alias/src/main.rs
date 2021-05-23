#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book
fn borrow_book(book: &Book) {
    // book.year = 2014; Cannot do this
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// This function takes a reference to a mutable book and changes `year` to 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // Create an immutable Book named `immutabook`
    let immutabook = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    let mut immutabook2 = Book {
        // string literals have type `&'static str`
        author: "miki",
        title: "something",
        year: 1992,
    };

    borrow_book(&immutabook);
    borrow_book(&immutabook2);

    // Create a mutable copy of `immutabook` and call it `mutabook`
    let mut mutabook = immutabook;
    let mutabook2 = &mut immutabook2;
    // let immutabook2 = &mut immutabook;
    let immutabook3 = immutabook;
    // let mutabook2 = &mut immutabook;

    mutabook.year = 2021;
    mutabook2.year = 2000;
    // Immutably borrow an immutable object
    // borrow_book2(immutabook2);
    borrow_book(&immutabook);
    borrow_book(&immutabook2);

    // Immutably borrow a mutable object
    borrow_book(&mutabook);
    
    // Borrow a mutable object as mutable
    new_edition(&mut mutabook);
    
    // Error! Cannot borrow an immutable object as mutable
    // new_edition(&mut immutabook);
    // FIXME ^ Comment out this line
}
