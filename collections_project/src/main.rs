use std::io;

pub mod book;
pub mod library;

use book::Book;
use library::Library;

fn main() {
    println!("📚 Kişisel Kütüphane Yönetim Sistemi");
    println!("=====================================");

    let mut library = Library::new();
    
    // Örnek kitaplar ekle
    add_sample_books(&mut library);

    loop {
        print_menu();
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Girdi okunamadı");

        match input.trim() {
            "1" => add_book_interactive(&mut library),
            "2" => list_books(&library),
            "3" => search_book_interactive(&library),
            "4" => borrow_book_interactive(&mut library),
            "5" => return_book_interactive(&mut library),
            "6" => show_stats(&library),
            "7" => show_borrow_history(&library),
            "8" => search_by_author_interactive(&library),
            "9" => remove_book_interactive(&mut library),
            "0" => {
                println!("👋 Program sonlandırılıyor...");
                break;
            }
            _ => println!("❌ Geçersiz seçenek! Lütfen 0-9 arası bir sayı girin."),
        }
        
        println!("\nDevam etmek için Enter'a basın...");
        let mut pause = String::new();
        io::stdin().read_line(&mut pause).unwrap();
    }
}

fn print_menu() {
    println!("\n📋 MENÜ:");
    println!("1. 📖 Kitap Ekle");
    println!("2. 📚 Tüm Kitapları Listele");
    println!("3. 🔍 Kitap Ara");
    println!("4. 📤 Kitap Ödünç Ver");
    println!("5. 📥 Kitap İade Al");
    println!("6. 📊 İstatistikler");
    println!("7. 📋 Ödünç Alma Geçmişi");
    println!("8. 👤 Yazara Göre Ara");
    println!("9. 🗑️  Kitap Sil");
    println!("0. 🚪 Çıkış");
    print!("Seçiminiz: ");
}

fn add_sample_books(library: &mut Library) {
    let books = vec![
        Book::new("Rust Programlama Dili", "Steve Klabnik", "978-1234567890", 2018),
        Book::new("JavaScript: The Good Parts", "Douglas Crockford", "978-0596517748", 2008),
        Book::new("Clean Code", "Robert C. Martin", "978-0132350884", 2008),
        Book::new("Design Patterns", "Gang of Four", "978-0201633610", 1994),
        Book::new("Algorithms", "Robert Sedgewick", "978-0321573513", 2011),
    ];

    for book in books {
        library.add_book(book);
    }
    
    println!("✅ 5 örnek kitap kütüphaneye eklendi!");
}

fn add_book_interactive(library: &mut Library) {
    println!("\n📖 YENİ KİTAP EKLEME");
    println!("===================");
    
    let title = get_input("Kitap başlığı: ");
    let author = get_input("Yazar: ");
    let isbn = get_input("ISBN: ");
    
    let year: u32 = loop {
        match get_input("Yayın yılı: ").parse() {
            Ok(y) => break y,
            Err(_) => println!("❌ Lütfen geçerli bir yıl girin!"),
        }
    };

    let book = Book::new(&title, &author, &isbn, year);
    let result = library.add_book(book);
    println!("✅ {}", result);
}

fn list_books(library: &Library) {
    println!("\n📚 TÜM KİTAPLAR");
    println!("================");
    
    let books = library.list_all_books();
    
    if books.is_empty() {
        println!("📭 Kütüphanede hiç kitap yok.");
        return;
    }

    let book_count = books.len(); // len() değerini önceden al
    for book in books {
        println!("{}", book.get_full_info());
    }
    
    println!("\n📊 Toplam {} kitap listelendi.", book_count);
}

fn search_book_interactive(library: &Library) {
    println!("\n🔍 KİTAP ARAMA");
    println!("==============");
    
    let query = get_input("Arama kelimesi (başlık veya yazar): ");
    let results = library.search_books(&query);
    
    if results.is_empty() {
        println!("❌ '{}' için sonuç bulunamadı.", query);
        return;
    }
    
    println!("\n📋 {} sonuç bulundu:", results.len());
    for book in results {
        println!("{}", book.get_full_info());
    }
}

fn borrow_book_interactive(library: &mut Library) {
    println!("\n📤 KİTAP ÖDÜNÇ VERME");
    println!("====================");
    
    let isbn = get_input("Ödünç verilecek kitabın ISBN'i: ");
    
    match library.borrow_book(&isbn) {
        Ok(message) => println!("✅ {}", message),
        Err(error) => println!("❌ {}", error),
    }
}

fn return_book_interactive(library: &mut Library) {
    println!("\n📥 KİTAP İADE ALMA");
    println!("==================");
    
    let isbn = get_input("İade edilecek kitabın ISBN'i: ");
    
    match library.return_book(&isbn) {
        Ok(message) => println!("✅ {}", message),
        Err(error) => println!("❌ {}", error),
    }
}

fn show_stats(library: &Library) {
    println!("\n📊 KÜTÜPHANE İSTATİSTİKLERİ");
    println!("============================");
    
    let stats = library.get_stats();
    
    for (key, value) in &stats {
        println!("📈 {}: {}", key, value);
    }
}

fn show_borrow_history(library: &Library) {
    println!("\n📋 ÖDÜNÇ ALMA GEÇMİŞİ");
    println!("======================");
    
    let history = library.get_borrow_history();
    
    if history.is_empty() {
        println!("📭 Henüz hiç kitap ödünç verilmemiş.");
        return;
    }
    
    for (index, entry) in history.iter().enumerate() {
        println!("{}. {}", index + 1, entry);
    }
    
    println!("\n📊 Toplam {} ödünç alma işlemi.", history.len());
}

fn search_by_author_interactive(library: &Library) {
    println!("\n👤 YAZARA GÖRE ARAMA");
    println!("====================");
    
    let author = get_input("Yazar adı: ");
    let books = library.find_books_by_author(&author);
    
    if books.is_empty() {
        println!("❌ '{}' yazarına ait kitap bulunamadı.", author);
        return;
    }
    
    println!("\n📚 {} yazarının {} kitabı bulundu:", author, books.len());
    for book in books {
        println!("{}", book.get_full_info());
    }
}

fn remove_book_interactive(library: &mut Library) {
    println!("\n🗑️ KİTAP SİLME");
    println!("===============");
    
    let isbn = get_input("Silinecek kitabın ISBN'i: ");
    
    match library.remove_book(&isbn) {
        Ok(message) => println!("✅ {}", message),
        Err(error) => println!("❌ {}", error),
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    use std::io::Write;
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Girdi okunamadı");
    
    input.trim().to_string()
}
