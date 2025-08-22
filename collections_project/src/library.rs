use std::collections::HashMap;
use crate::book::Book;

// Kütüphane yönetim sistemi - HashMap ve Vector kullanımı
#[derive(Debug)]
pub struct Library {
    // HashMap: ISBN -> Book mapping
    books: HashMap<String, Book>,
    // Vector: Ödünç alınan kitapların geçmişi
    borrow_history: Vec<String>,
    // HashMap: Yazar -> kitap listesi mapping
    books_by_author: HashMap<String, Vec<String>>,
}

impl Library {
    // Yeni kütüphane oluşturma
    pub fn new() -> Library {
        Library {
            books: HashMap::new(),
            borrow_history: Vec::new(),
            books_by_author: HashMap::new(),
        }
    }

    // Kitap ekleme - HashMap kullanımı
    pub fn add_book(&mut self, book: Book) -> String {
        let isbn = book.isbn.clone();
        let author = book.author.clone();
        let title = book.title.clone();

        // Kitabı ana koleksiyona ekle
        self.books.insert(isbn.clone(), book);

        // Yazar indeksini güncelle
        self.books_by_author
            .entry(author.clone())
            .or_insert_with(Vec::new)
            .push(isbn.clone());

        format!("📚 '{}' kitabı kütüphaneye başarıyla eklendi!", title)
    }

    // Kitap arama - HashMap get kullanımı
    pub fn find_book_by_isbn(&self, isbn: &str) -> Option<&Book> {
        self.books.get(isbn)
    }

    // Yazara göre kitap arama - HashMap ve Vector kullanımı
    pub fn find_books_by_author(&self, author: &str) -> Vec<&Book> {
        match self.books_by_author.get(author) {
            Some(isbns) => {
                isbns
                    .iter()
                    .filter_map(|isbn| self.books.get(isbn))
                    .collect()
            }
            None => Vec::new(),
        }
    }

    // Tüm kitapları listele - HashMap values() kullanımı
    pub fn list_all_books(&self) -> Vec<&Book> {
        self.books.values().collect()
    }

    // Mevcut kitapları listele - HashMap values() ve filtering
    pub fn list_available_books(&self) -> Vec<&Book> {
        self.books
            .values()
            .filter(|book| book.available)
            .collect()
    }

    // Kitap ödünç verme - HashMap get_mut kullanımı
    pub fn borrow_book(&mut self, isbn: &str) -> Result<String, String> {
        match self.books.get_mut(isbn) {
            Some(book) => {
                match book.borrow_book() {
                    Ok(message) => {
                        // Ödünç alma geçmişine ekle
                        self.borrow_history.push(format!(
                            "{} - {} ({})",
                            book.title, book.author, isbn
                        ));
                        Ok(message)
                    }
                    Err(error) => Err(error),
                }
            }
            None => Err(format!("ISBN '{}' ile kitap bulunamadı.", isbn)),
        }
    }

    // Kitap iade etme
    pub fn return_book(&mut self, isbn: &str) -> Result<String, String> {
        match self.books.get_mut(isbn) {
            Some(book) => Ok(book.return_book()),
            None => Err(format!("ISBN '{}' ile kitap bulunamadı.", isbn)),
        }
    }

    // Ödünç alma geçmişini görüntüle - Vector kullanımı
    pub fn get_borrow_history(&self) -> &Vec<String> {
        &self.borrow_history
    }

    // İstatistikler
    pub fn get_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        let total_books = self.books.len();
        let available_books = self.books.values()
            .filter(|book| book.available)
            .count();
        let borrowed_books = total_books - available_books;
        let total_authors = self.books_by_author.len();

        stats.insert("Toplam Kitap".to_string(), total_books);
        stats.insert("Mevcut Kitap".to_string(), available_books);
        stats.insert("Ödünç Verilen".to_string(), borrowed_books);
        stats.insert("Toplam Yazar".to_string(), total_authors);
        stats.insert("Toplam Ödünç Alma".to_string(), self.borrow_history.len());

        stats
    }

    // Kitap silme - HashMap remove kullanımı
    pub fn remove_book(&mut self, isbn: &str) -> Result<String, String> {
        match self.books.remove(isbn) {
            Some(book) => {
                // Yazar indeksinden de kaldır
                if let Some(author_books) = self.books_by_author.get_mut(&book.author) {
                    author_books.retain(|book_isbn| book_isbn != isbn);
                    // Eğer yazarın başka kitabı kalmadıysa yazarı da kaldır
                    if author_books.is_empty() {
                        self.books_by_author.remove(&book.author);
                    }
                }
                Ok(format!("'{}' kitabı kütüphaneden kaldırıldı.", book.title))
            }
            None => Err(format!("ISBN '{}' ile kitap bulunamadı.", isbn)),
        }
    }

    // Kelime araması - String contains kullanımı
    pub fn search_books(&self, query: &str) -> Vec<&Book> {
        let query_lower = query.to_lowercase();
        self.books
            .values()
            .filter(|book| {
                book.title.to_lowercase().contains(&query_lower) ||
                book.author.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
}
