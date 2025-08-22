pub mod book;
pub mod library;

#[cfg(test)]
mod tests {
    use crate::book::Book;
    use crate::library::Library;

    #[test]
    fn test_create_book() {
        let book = Book::new("Test Book", "Test Author", "123456789", 2024);
        assert_eq!(book.title, "Test Book");
        assert_eq!(book.author, "Test Author");
        assert_eq!(book.isbn, "123456789");
        assert_eq!(book.year, 2024);
        assert_eq!(book.available, true);
    }

    #[test]
    fn test_book_borrow_and_return() {
        let mut book = Book::new("Test Book", "Test Author", "123456789", 2024);
        
        // Kitap ödünç verme
        let result = book.borrow_book();
        assert!(result.is_ok());
        assert_eq!(book.available, false);
        
        // İkinci kez ödünç vermeye çalışma
        let result = book.borrow_book();
        assert!(result.is_err());
        
        // Kitap iade etme
        book.return_book();
        assert_eq!(book.available, true);
    }

    #[test]
    fn test_library_add_and_find_book() {
        let mut library = Library::new();
        let book = Book::new("Test Book", "Test Author", "123456789", 2024);
        
        library.add_book(book);
        
        let found_book = library.find_book_by_isbn("123456789");
        assert!(found_book.is_some());
        assert_eq!(found_book.unwrap().title, "Test Book");
    }

    #[test]
    fn test_library_search_functionality() {
        let mut library = Library::new();
        
        library.add_book(Book::new("Rust Programming", "Steve Klabnik", "111", 2018));
        library.add_book(Book::new("JavaScript Guide", "Mozilla", "222", 2020));
        library.add_book(Book::new("Python Basics", "John Doe", "333", 2021));
        
        // Başlık ile arama
        let results = library.search_books("rust");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Rust Programming");
        
        // Yazar ile arama
        let results = library.search_books("john");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].author, "John Doe");
        
        // Bulunamayan arama
        let results = library.search_books("nonexistent");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_library_borrow_history() {
        let mut library = Library::new();
        library.add_book(Book::new("Test Book", "Test Author", "123", 2024));
        
        // İlk başta geçmiş boş olmalı
        assert_eq!(library.get_borrow_history().len(), 0);
        
        // Kitap ödünç verme
        let result = library.borrow_book("123");
        assert!(result.is_ok());
        
        // Geçmişte bir kayıt olmalı
        assert_eq!(library.get_borrow_history().len(), 1);
    }

    #[test]
    fn test_library_stats() {
        let mut library = Library::new();
        
        library.add_book(Book::new("Book 1", "Author 1", "111", 2020));
        library.add_book(Book::new("Book 2", "Author 2", "222", 2021));
        library.add_book(Book::new("Book 3", "Author 1", "333", 2022));
        
        let stats = library.get_stats();
        
        assert_eq!(*stats.get("Toplam Kitap").unwrap(), 3);
        assert_eq!(*stats.get("Mevcut Kitap").unwrap(), 3);
        assert_eq!(*stats.get("Ödünç Verilen").unwrap(), 0);
        assert_eq!(*stats.get("Toplam Yazar").unwrap(), 2);
        
        // Bir kitap ödünç ver
        library.borrow_book("111").unwrap();
        
        let stats = library.get_stats();
        assert_eq!(*stats.get("Mevcut Kitap").unwrap(), 2);
        assert_eq!(*stats.get("Ödünç Verilen").unwrap(), 1);
        assert_eq!(*stats.get("Toplam Ödünç Alma").unwrap(), 1);
    }

    #[test]
    fn test_string_operations() {
        let mut book = Book::new("Original Title", "Original Author", "123", 2024);
        
        // String güncelleme
        book.update_title("New Title");
        book.update_author("New Author");
        
        assert_eq!(book.title, "New Title");
        assert_eq!(book.author, "New Author");
        
        // String formatting
        let info = book.get_full_info();
        assert!(info.contains("New Title"));
        assert!(info.contains("New Author"));
    }

    #[test]
    fn test_vector_operations() {
        let mut library = Library::new();
        
        // Vector'e kitap ekleme
        library.add_book(Book::new("Book 1", "Author A", "111", 2020));
        library.add_book(Book::new("Book 2", "Author B", "222", 2021));
        library.add_book(Book::new("Book 3", "Author A", "333", 2022));
        
        // Tüm kitapları listeleme
        let all_books = library.list_all_books();
        assert_eq!(all_books.len(), 3);
        
        // Yazara göre kitap bulma
        let author_books = library.find_books_by_author("Author A");
        assert_eq!(author_books.len(), 2);
        
        // Mevcut kitapları listeleme
        library.borrow_book("111").unwrap(); // Bir kitap ödünç ver
        let available_books = library.list_available_books();
        assert_eq!(available_books.len(), 2);
    }

    #[test]
    fn test_hashmap_operations() {
        let mut library = Library::new();
        
        // HashMap'e kitap ekleme
        let book = Book::new("Test Book", "Test Author", "123456", 2024);
        library.add_book(book);
        
        // HashMap'ten kitap bulma
        assert!(library.find_book_by_isbn("123456").is_some());
        assert!(library.find_book_by_isbn("nonexistent").is_none());
        
        // HashMap'ten kitap silme
        let result = library.remove_book("123456");
        assert!(result.is_ok());
        assert!(library.find_book_by_isbn("123456").is_none());
        
        // Olmayan kitap silmeye çalışma
        let result = library.remove_book("nonexistent");
        assert!(result.is_err());
    }
}
