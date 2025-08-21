// 📚 Kütüphane Yönetim Sistemi - Ownership Öğrenme Projesi
// Bu projede Rust'ın ownership, borrowing ve slice kavramlarını öğreneceğiz

mod book;
mod library;

use book::Book;
use library::Library;

fn main() {
    println!("🚀 Kütüphane Yönetim Sistemi - Ownership Dersleri");
    println!("{}", "=".repeat(50));
    
    // Adım 1: Ownership - Sahiplik kavramını öğrenelim
    ownership_basics();
    
    // Adım 2: Move semantics - Taşıma semantiği
    move_semantics_demo();
    
    // Adım 3: Borrowing - Borç alma
    borrowing_demo();
    
    // Adım 4: Mutable references - Değiştirilebilir referanslar
    mutable_references_demo();
    
    // Adım 5: Slice types - Dilim türleri
    slice_demo();
}

// ADIM 1: Temel Ownership Kavramları
fn ownership_basics() {
    println!("\n📖 ADIM 1: Temel Ownership Kavramları");
    println!("{}", "-".repeat(40));
    
    // String ownership - sahiplik
    let book_title = String::from("1984");
    println!("Kitap başlığı: {}", book_title);
    
    // Ownership transfer (move)
    let transferred_title = book_title;
    println!("Transfer edilmiş başlık: {}", transferred_title);
    
    // ⚠️ Aşağıdaki satır hata verir çünkü book_title artık geçerli değil
    // println!("Orijinal başlık: {}", book_title); // ERROR!
    
    println!("✅ Ownership başarıyla transfer edildi!");
}

// ADIM 2: Move Semantics
fn move_semantics_demo() {
    println!("\n📦 ADIM 2: Move Semantics - Taşıma Semantiği");
    println!("{}", "-".repeat(40));
    
    let book1 = Book::new("Suç ve Ceza".to_string(), "Dostoyevski".to_string());
    println!("Kitap 1 oluşturuldu: {}", book1.get_title());
    
    // Move occurs here
    let book2 = book1;
    println!("Kitap 2'ye taşındı: {}", book2.get_title());
    
    // book1 artık kullanılamaz
    // println!("{}", book1.get_title()); // ERROR!
    
    println!("✅ Move semantics tamamlandı!");
}

// ADIM 3: Borrowing - Borç Alma
fn borrowing_demo() {
    println!("\n🤝 ADIM 3: Borrowing - Borç Alma");
    println!("{}", "-".repeat(40));
    
    let book = Book::new("Don Kişot".to_string(), "Cervantes".to_string());
    
    // Immutable borrow
    print_book_info(&book);
    
    // Hala book'u kullanabiliriz
    println!("Kitap hala burada: {}", book.get_title());
    
    println!("✅ Borrowing başarıyla tamamlandı!");
}

// ADIM 4: Mutable References
fn mutable_references_demo() {
    println!("\n🔄 ADIM 4: Mutable References - Değiştirilebilir Referanslar");
    println!("{}", "-".repeat(40));
    
    let mut library = Library::new();
    
    println!("Kütüphane oluşturuldu, kitap sayısı: {}", library.book_count());
    
    // Mutable borrow
    add_books_to_library(&mut library);
    
    println!("Kitaplar eklendikten sonra sayı: {}", library.book_count());
    println!("✅ Mutable references tamamlandı!");
}

// ADIM 5: Slice Types
fn slice_demo() {
    println!("\n🔪 ADIM 5: Slice Types - Dilim Türleri");
    println!("{}", "-".repeat(40));
    
    let book_title = "Yüzüklerin Efendisi: Yüzük Kardeşliği";
    
    // String slice
    let first_part = get_first_word(book_title);
    println!("İlk kelime: '{}'", first_part);
    
    // Array slice
    let page_numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let first_five_pages = &page_numbers[0..5];
    println!("İlk 5 sayfa: {:?}", first_five_pages);
    
    println!("✅ Slice types tamamlandı!");
}

// Yardımcı fonksiyonlar
fn print_book_info(book: &Book) {
    println!("📚 Kitap Bilgisi: {} - {}", book.get_title(), book.get_author());
}

fn add_books_to_library(library: &mut Library) {
    library.add_book(Book::new("Savaş ve Barış".to_string(), "Tolstoy".to_string()));
    library.add_book(Book::new("Beyaz Diş".to_string(), "Jack London".to_string()));
    library.add_book(Book::new("Simyacı".to_string(), "Paulo Coelho".to_string()));
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}