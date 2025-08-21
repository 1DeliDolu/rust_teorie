// 🏛️ Library struct - Kütüphane yapısı
// Bu struct borrowing ve mutable references kavramlarını gösterecek

use crate::book::Book;

pub struct Library {
    books: Vec<Book>,
    name: String,
}

impl Library {
    // Constructor
    pub fn new() -> Library {
        Library {
            books: Vec::new(),
            name: String::from("Merkez Kütüphane"),
        }
    }
    
    pub fn new_with_name(name: String) -> Library {
        Library {
            books: Vec::new(),
            name,
        }
    }
    
    // Add book - takes ownership
    pub fn add_book(&mut self, book: Book) {
        println!("📚 Kütüphaneye kitap ekleniyor: {}", book.get_title());
        self.books.push(book);
    }
    
    // Borrow book - returns reference
    pub fn find_book(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|book| book.get_title() == title)
    }
    
    // Mutable borrow
    pub fn find_book_mut(&mut self, title: &str) -> Option<&mut Book> {
        self.books.iter_mut().find(|book| book.get_title() == title)
    }
    
    // Remove book - transfers ownership back
    pub fn remove_book(&mut self, title: &str) -> Option<Book> {
        if let Some(pos) = self.books.iter().position(|book| book.get_title() == title) {
            Some(self.books.remove(pos))
        } else {
            None
        }
    }
    
    // Count books - no ownership transfer
    pub fn book_count(&self) -> usize {
        self.books.len()
    }
    
    // Get all book titles - returns new owned data
    pub fn get_all_titles(&self) -> Vec<String> {
        self.books.iter()
            .map(|book| book.get_title().to_string())
            .collect()
    }
    
    // Get books slice - returns reference to slice
    pub fn get_books_slice(&self) -> &[Book] {
        &self.books
    }
    
    // Get mutable books slice
    pub fn get_books_slice_mut(&mut self) -> &mut [Book] {
        &mut self.books
    }
    
    // Library name getter
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    // Print all books - demonstrates borrowing
    pub fn print_all_books(&self) {
        println!("\n📚 {} - Tüm Kitaplar:", self.name);
        println!("{}", "-".repeat(30));
        
        if self.books.is_empty() {
            println!("Kütüphanede hiç kitap yok.");
            return;
        }
        
        for (index, book) in self.books.iter().enumerate() {
            println!("{}. {}", index + 1, book);
        }
    }
    
    // Search books by author - demonstrates string slices
    pub fn find_books_by_author(&self, author: &str) -> Vec<&Book> {
        self.books.iter()
            .filter(|book| book.get_author().contains(author))
            .collect()
    }
}