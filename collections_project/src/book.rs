// Kitap veri yapısı - String kullanımına örnek
#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub year: u32,
    pub available: bool,
}

impl Book {
    // Yeni kitap oluşturma - String ile çalışma
    pub fn new(title: &str, author: &str, isbn: &str, year: u32) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            isbn: String::from(isbn),
            year,
            available: true,
        }
    }

    // Kitap bilgilerini güncelleme - String manipülasyonu
    pub fn update_title(&mut self, new_title: &str) {
        self.title = String::from(new_title);
    }

    pub fn update_author(&mut self, new_author: &str) {
        self.author = String::from(new_author);
    }

    // Kitap bilgilerini formatlamak - String concatenation
    pub fn get_full_info(&self) -> String {
        format!(
            "📖 {} - {} ({}), ISBN: {}, Durum: {}",
            self.title,
            self.author,
            self.year,
            self.isbn,
            if self.available { "Mevcut" } else { "Ödünç verildi" }
        )
    }

    // Kitap ödünç verme durumu
    pub fn borrow_book(&mut self) -> Result<String, String> {
        if self.available {
            self.available = false;
            Ok(format!("'{}' kitabı başarıyla ödünç verildi.", self.title))
        } else {
            Err(format!("'{}' kitabı zaten ödünç verilmiş.", self.title))
        }
    }

    pub fn return_book(&mut self) -> String {
        self.available = true;
        format!("'{}' kitabı başarıyla iade edildi.", self.title)
    }
}
