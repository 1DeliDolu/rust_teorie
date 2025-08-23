## 🔄 Değiştirilebilirlik (mutability)

Değiştirilebilir veriler `&mut T` kullanılarak **değiştirilebilir şekilde ödünç alınabilir (mutable borrow)**. Buna **değiştirilebilir referans (mutable reference)** denir ve ödünç alan tarafa hem okuma hem de yazma erişimi sağlar.

Buna karşılık, `&T` ile yapılan ödünç alma **değiştirilemez referans (immutable reference)** oluşturur ve ödünç alan yalnızca veriyi okuyabilir, fakat değiştiremez.

```rust
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str`, salt okunur bellekte ayrılmış bir string’e referanstır
    author: &'static str,
    title: &'static str,
    year: u32,
}

// Bu fonksiyon bir kitabı değiştirilemez referansla ödünç alır
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// Bu fonksiyon bir kitabı değiştirilebilir referansla ödünç alır
// ve `year` alanını 2014 olarak değiştirir
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // `immutabook` adında değiştirilemez bir Book oluştur
    let immutabook = Book {
        // string literalleri `&'static str` tipindedir
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // `immutabook`’un değiştirilebilir bir kopyasını oluştur ve `mutabook` adını ver
    let mut mutabook = immutabook;
    
    // Değiştirilemez bir nesneyi değiştirilemez şekilde ödünç al
    borrow_book(&immutabook);

    // Değiştirilebilir bir nesneyi değiştirilemez şekilde ödünç al
    borrow_book(&mutabook);
    
    // Değiştirilebilir bir nesneyi değiştirilebilir şekilde ödünç al
    new_edition(&mut mutabook);
    
    // Hata! Değiştirilemez bir nesne değiştirilebilir şekilde ödünç alınamaz
    new_edition(&mut immutabook);
    // FIXME ^ Bu satırı yorum satırı yapın
}
```

👉 Bu örnekte, `immutabook` değiştirilemez tanımlandığı için `&mut immutabook` kullanılamaz. Ancak `mutabook` değiştirilebilir tanımlandığı için hem `&mut` hem de `&` ile ödünç alınabilir.
