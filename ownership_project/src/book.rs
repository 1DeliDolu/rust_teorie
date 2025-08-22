// 📚 Book struct - Kitap yapısı
// Bu struct ownership kavramlarını göstermek için kullanılacak

#[derive(Debug, Clone)]
pub struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

impl Book {
    // Constructor - Yeni kitap oluştur
    pub fn new(title: String, author: String) -> Book {
        Book {
            title,
            author,
            pages: 0,
            available: true,
        }
    }
    
    // Getter methods - Ownership kurallarına uygun erişim
    pub fn get_title(&self) -> &str {
        &self.title  // String slice döndürür, ownership transfer etmez
    }
    
    pub fn get_author(&self) -> &str {
        &self.author
    }
    
    pub fn get_pages(&self) -> u32 {
        self.pages  // u32 Copy trait'ine sahip, ownership transfer olmaz
    }
    
    pub fn is_available(&self) -> bool {
        self.available
    }
    
    // Setter methods - Mutable borrows
    pub fn set_pages(&mut self, pages: u32) {
        self.pages = pages;
    }
    
    pub fn set_available(&mut self, available: bool) {
        self.available = available;
    }
    
    // Method that takes ownership
    pub fn destroy_book(self) -> String {
        println!("📖 Kitap yok ediliyor: {}", self.title);
        self.title  // Ownership of title is moved out
    }
    
    // Method that creates a new owned String
    pub fn get_full_info(&self) -> String {
        format!("{} - {} ({} sayfa)", self.title, self.author, self.pages)
    }
}

// Display trait implementation
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "📚 {} by {}", self.title, self.author)
    }
}