# 📚 Collections Project - Kişisel Kütüphane Yönetim Sistemi

Bu proje, Rust'ın **8. bölüm - Common Collections** konularını pratik yaparak öğrenmek için geliştirilmiş bir kütüphane yönetim sistemidir.

## 🎯 Proje Amacı

Bu projede Rust'ın üç temel koleksiyon yapısını öğreneceksiniz:

- **📋 Vector (`Vec<T>`)**: Dinamik listeler
- **🔤 String**: UTF-8 kodlu metin işleme
- **📦 HashMap**: Anahtar-değer çiftleri

## 🚀 Özellikler

### 📖 Kitap Yönetimi

- ✅ Yeni kitap ekleme
- ✅ Kitap bilgilerini güncelleme
- ✅ Kitap silme
- ✅ Kitap arama (başlık ve yazar)

### 📚 Kütüphane Operasyonları

- ✅ Kitap ödünç verme/iade
- ✅ Ödünç alma geçmişi
- ✅ Yazara göre kitap listeleme
- ✅ İstatistik gösterme

### 🔍 Arama Fonksiyonları

- ✅ ISBN ile kitap bulma
- ✅ Metin tabanlı arama
- ✅ Yazar bazında gruplandırma

## 📁 Proje Yapısı

```
collections_project/
├── src/
│   ├── main.rs          # Ana program ve kullanıcı arayüzü
│   ├── lib.rs           # Test dosyası
│   ├── book.rs          # Kitap veri yapısı ve String işlemleri
│   └── library.rs       # Kütüphane yönetimi - Vector ve HashMap
├── Cargo.toml           # Proje yapılandırması
└── README.md           # Bu dosya
```

## 🏃‍♂️ Çalıştırma

### Programı Çalıştırma

```bash
cd collections_project
cargo run
```

### Testleri Çalıştırma

```bash
cargo test
```

### Dökümantasyon Oluşturma

```bash
cargo doc --open
```

## 📚 Öğrenilecek Konular

### 1. Vector (`Vec<T>`) Kullanımı

- `Vec::new()` ile yeni vektör oluşturma
- `push()` ile eleman ekleme
- `get()` ve indeksleme ile erişim
- `iter()` ile döngü yapma
- `filter()`, `map()`, `collect()` fonksiyonel programlama

### 2. String İşlemleri

- `String::new()` ve `String::from()` kullanımı
- `push_str()` ve `push()` ile ekleme
- `format!()` makrosu ile formatlama
- `to_lowercase()`, `contains()` gibi yöntemler
- UTF-8 karakter desteği

### 3. HashMap Kullanımı

- `HashMap::new()` ile oluşturma
- `insert()` ile veri ekleme
- `get()` ve `get_mut()` ile erişim
- `entry()` API kullanımı
- `keys()`, `values()`, `iter()` ile döngü

## 💡 Kod Örnekleri

### Vector Kullanımı

```rust
// Yeni vektör oluşturma
let mut books = Vec::new();
books.push(book);

// Filtreleme
let available_books: Vec<&Book> = books
    .iter()
    .filter(|book| book.available)
    .collect();
```

### HashMap Kullanımı

```rust
// HashMap oluşturma
let mut books = HashMap::new();
books.insert(isbn.clone(), book);

// Değer alma
if let Some(book) = books.get(&isbn) {
    println!("Kitap bulundu: {}", book.title);
}
```

### String İşlemleri

```rust
// String oluşturma
let title = String::from("Rust Programming");

// Formatlama
let info = format!("📖 {} - {} ({})", title, author, year);

// Arama
if title.to_lowercase().contains(&query.to_lowercase()) {
    // Bulundu!
}
```

## 🧪 Test Senaryoları

Proje şu test senaryolarını içerir:

- ✅ Kitap oluşturma ve özellik testleri
- ✅ Ödünç alma/iade işlemleri
- ✅ Arama fonksiyonalitesi
- ✅ Vector operasyonları
- ✅ HashMap operasyonları
- ✅ String manipülasyonu

## 📋 Adım Adım Öğrenme Süreci

### Adım 1: Temel Yapıları Anlayın

1. `book.rs` dosyasını inceleyin - String kullanımı
2. `library.rs` dosyasını inceleyin - HashMap ve Vector kullanımı

### Adım 2: Interaktif Olarak Test Edin

1. `cargo run` ile programı çalıştırın
2. Menüdeki tüm seçenekleri deneyın
3. Hata durumlarını test edin

### Adım 3: Testleri Çalıştırın

1. `cargo test` ile unit testleri çalıştırın
2. Her test fonksiyonunu ayrı ayrı inceleyin

### Adım 4: Kodu Geliştirin

1. Yeni özellikler ekleyin
2. Daha fazla test yazın
3. Performance iyileştirmeleri yapın

## 🔧 Geliştirme Fikirleri

Bu projeyi şu şekillerde geliştirebilirsiniz:

- 📅 Tarih bazlı filtreleme
- 📊 Daha detaylı istatistikler
- 💾 Dosyaya kaydetme/yükleme
- 🏷️ Kategori sistemi
- 📱 JSON formatında veri exchange

## 📚 Rust Collections - Önemli Noktalar

### Vector

- Heap'te dinamik boyutlu dizi
- Aynı tip elemanları tutar
- Index tabanlı erişim
- Otomatik büyür/küçülür

### String

- UTF-8 encoded bytes
- Heap'te saklanır
- Mutable/immutable variants
- Rich text processing API

### HashMap

- Key-value storage
- O(1) average access time
- Owned keys and values
- Iterator support

Bu proje ile Rust'ın collection yapılarını derinlemesine öğrenecek ve gerçek dünya uygulamalarında nasıl kullanılacağını deneyimleyeceksiniz! 🚀

---

## 🎯 Collections Project - Tam Adım Adım Öğrenme Rehberi

### 📋 **PHASE 1: Proje Keşfi & Kurulum** _(5-10 dakika)_

<details>
<summary>🔽 <strong>1.1 Proje Dosyalarını Keşfedin</strong></summary>

```
collections_project/
├── 🎮 src/main.rs          # İnteraktif menü sistemi
├── 🧪 src/lib.rs           # Kapsamlı test süiti
├── 📖 src/book.rs          # String işlemlerinin kalbi
├── 📚 src/library.rs       # HashMap + Vector masterclass
├── 💡 examples/            # Pratik örnekler koleksiyonu
└── 📚 README.md           # Bu rehber
```

**✅ Yapılacaklar:**

- [ ] Tüm dosyaları VS Code'da açın
- [ ] Syntax highlighting'i kontrol edin
- [ ] Kod yapısını genel olarak inceleyin

</details>

<details>
<summary>🔽 <strong>1.2 İlk Çalıştırma</strong></summary>

```bash
# Terminal'de çalıştırın
cd collections_project
cargo check    # Syntax kontrolü
cargo build    # Derleme
cargo run      # Ana program
```

**🎯 Beklenen Çıktı:**

```
📚 Kişisel Kütüphane Yönetim Sistemi
=====================================
✅ 5 örnek kitap kütüphaneye eklendi!

📋 MENÜ:
1. 📖 Kitap Ekle
2. 📚 Tüm Kitapları Listele
...
```

</details>

---

### 📋 **PHASE 2: Collections Teorisini Kod İle Öğrenme** _(20-30 dakika)_

<details>
<summary>🔽 <strong>2.1 Vector Deep Dive</strong> 📊</summary>

**📍 Odak: `src/library.rs` - satır 15-150**

```rust
// 🔍 Bu kod parçalarını bulup inceleyin:

// 1️⃣ Vector Oluşturma
let mut borrow_history: Vec<String> = Vec::new();

// 2️⃣ Vector'e Ekleme
self.borrow_history.push(format!("..."));

// 3️⃣ Vector Filtreleme
let available_books: Vec<&Book> = self.books
    .values()
    .filter(|book| book.available)
    .collect();
```

**✅ Pratik Görevler:**

- [ ] `list_all_books()` fonksiyonunu inceleyin
- [ ] `get_borrow_history()` nasıl çalışıyor?
- [ ] `find_books_by_author()` Vector manipülasyonu
- [ ] Iterator chain'leri (`filter`, `map`, `collect`) anlayın

**🧠 Öğrenme Noktaları:**

- Vector ownership kuralları
- `&Vec` vs `Vec<&T>` farkı
- Iterator lazy evaluation

</details>

<details>
<summary>🔽 <strong>2.2 String Mastery</strong> 🔤</summary>

**📍 Odak: `src/book.rs` - tüm dosya**

```rust
// 🔍 Bu String operasyonlarını inceleyin:

// 1️⃣ String Oluşturma Yolları
String::from(title)     // Literal'den
title.to_string()       // &str'den
String::new()           // Boş string

// 2️⃣ String Manipülasyon
format!("📖 {} - {} ({})", title, author, year)
title.to_lowercase().contains(&query.to_lowercase())

// 3️⃣ Ownership & Borrowing
fn update_title(&mut self, new_title: &str) {
    self.title = String::from(new_title);
}
```

**✅ Pratik Görevler:**

- [ ] `get_full_info()` formatlamayı çalıştırın
- [ ] `update_title()` vs `update_author()` karşılaştırın
- [ ] UTF-8 karakterlerle test edin (Türkçe, emoji)
- [ ] String concatenation vs formatting performansını düşünün

**🧠 Öğrenme Noktaları:**

- `String` vs `&str` lifetime farkları
- UTF-8 encoding ve memory layout
- Format macro'larının gücü

</details>

<details>
<summary>🔽 <strong>2.3 HashMap Deep Dive</strong> 📦</summary>

**📍 Odak: `src/library.rs` - HashMap operasyonları**

```rust
// 🔍 Bu HashMap pattern'lerini inceleyin:

// 1️⃣ Temel CRUD Operations
books.insert(isbn.clone(), book);              // Create
books.get(&isbn)                               // Read
books.get_mut(&isbn)                           // Update
books.remove(&isbn)                            // Delete

// 2️⃣ Entry API (Advanced)
self.books_by_author
    .entry(author.clone())
    .or_insert_with(Vec::new)
    .push(isbn.clone());

// 3️⃣ Iteration Patterns
for (key, value) in &self.books { ... }
```

**✅ Pratik Görevler:**

- [ ] `add_book()` fonksiyonundaki double indexing
- [ ] `find_book_by_isbn()` vs `find_books_by_author()`
- [ ] `get_stats()` HashMap iteration
- [ ] `remove_book()` cascade deletion

**🧠 Öğrenme Noktaları:**

- Hash function ve collision handling
- Entry API'nin memory efficiency
- Key ownership (clone vs reference)

</details>

---

### 📋 **PHASE 3: İnteraktif Deneyim & Testing** _(15-20 dakika)_

<details>
<summary>🔽 <strong>3.1 Kullanıcı Senaryoları</strong></summary>

**🎮 Ana programı çalıştırıp şunları deneyin:**

```bash
cargo run
```

**📝 Test Senaryoları:**

1. **Kitap Ekleme Testi:**

   - [ ] Menü: `1` → Yeni kitap ekleyin
   - [ ] Türkçe karakterler deneyin: "Çalıkuşu", "Zülfü Livaneli"
   - [ ] Uzun başlık/yazar isimleri

2. **Arama Sistemi Testi:**

   - [ ] Menü: `3` → "rust" araması
   - [ ] Menü: `8` → "Steve Klabnik" yazar araması
   - [ ] Case-insensitive test: "RUST" vs "rust"

3. **Ödünç Alma Sistemi:**

   - [ ] Menü: `4` → ISBN: "978-1234567890"
   - [ ] Menü: `7` → Geçmişi kontrol
   - [ ] Menü: `5` → İade işlemi

4. **Hata Durumları:**
   - [ ] Olmayan ISBN ile ödünç verme
   - [ ] Zaten ödünç verilmiş kitap
   - [ ] Boş arama sorgusu

</details>

<details>
<summary>🔽 <strong>3.2 Unit Test Deep Dive</strong></summary>

```bash
cargo test -- --nocapture  # Detaylı output
cargo test test_vector_operations  # Specific test
```

**🧪 Test Kategorileri Analizi:**

- [ ] `test_create_book` - Struct initialization
- [ ] `test_library_search_functionality` - String matching
- [ ] `test_hashmap_operations` - Key-value operations
- [ ] `test_vector_operations` - Collection filtering

**💡 Test-Driven Learning:**
Her testten sonra kodda ilgili bölümü bulun ve nasıl çalıştığını anlayın.

</details>

---

### 📋 **PHASE 4: Pratik Örnekler Çalıştırma** _(10-15 dakika)_

<details>
<summary>🔽 <strong>4.1 Collections Examples</strong></summary>

```bash
cargo run --example collections_examples
```

**📊 Bu örneklerden öğreneceğiniz:**

- Vector functional programming patterns
- String UTF-8 işlemleri
- HashMap advanced usage patterns
- Real-world combination scenarios

**✅ Özel Dikkat:**

- `word_count` HashMap example
- Student grades system (nested collections)
- Document indexing system

</details>

---

### 📋 **PHASE 5: Kendi Geliştirmeleriniz** _(20+ dakika)_

<details>
<summary>🔽 <strong>5.1 Challenge Tasks</strong></summary>

**🏆 Beginner Challenges:**

- [ ] Kitap kategorisi sistemi ekleyin (`HashMap<String, Vec<String>>`)
- [ ] Yıl bazlı filtreleme (`Vec` filtering)
- [ ] Kitap rating sistemi (`HashMap` update)

**🏆 Intermediate Challenges:**

- [ ] JSON export/import (`serde` integration)
- [ ] Advanced search (multiple criteria)
- [ ] Performance benchmarking

**🏆 Advanced Challenges:**

- [ ] Custom collection type
- [ ] Memory optimization
- [ ] Concurrent access (Arc<Mutex<>>)

</details>

<details>
<summary>🔽 <strong>5.2 Code Quality Improvements</strong></summary>

```bash
# Code quality tools
cargo fmt      # Formatting
cargo clippy   # Linting
cargo doc --open  # Documentation
```

**📝 Improvement Ideas:**

- [ ] Error handling iyileştirmeleri
- [ ] More comprehensive tests
- [ ] Documentation comments
- [ ] Performance optimizations

</details>

---

### 🎯 **PHASE 6: Öğrenme Kontrolü & Next Steps**

<details>
<summary>🔽 <strong>6.1 Self Assessment</strong></summary>

**✅ Vector Mastery Check:**

- [ ] `Vec::new()` vs `vec![]` ne zaman hangisi?
- [ ] `push()` vs `extend()` performance farkı?
- [ ] Iterator chain optimization anlayışı?
- [ ] Ownership vs borrowing in collections?

**✅ String Mastery Check:**

- [ ] `String` vs `&str` memory layout?
- [ ] UTF-8 encoding implications?
- [ ] `format!` vs string concatenation?
- [ ] String slicing safety?

**✅ HashMap Mastery Check:**

- [ ] Entry API advanced patterns?
- [ ] Key requirements (Hash + Eq)?
- [ ] Performance characteristics?
- [ ] Memory usage optimization?

</details>

<details>
<summary>🔽 <strong>6.2 Next Learning Path</strong></summary>

**🚀 Recommended Next Topics:**

1. **Error Handling** (Chapter 9)
2. **Generic Types** (Chapter 10)
3. **Lifetimes** (Chapter 10)
4. **Testing** (Chapter 11)

**📚 Advanced Collections:**

- `BTreeMap` vs `HashMap`
- `VecDeque` for double-ended queues
- `HashSet` for unique collections
- Custom collection implementations

</details>

---

### 💡 **Pro Tips & Best Practices**

<details>
<summary>🔽 <strong>Performance Tips</strong></summary>

- 🚀 **Vector**: `with_capacity()` kullanın eğer size tahmin edebiliyorsanız
- 🚀 **String**: `format!` çok kullanımda, `push_str` alternatifi
- 🚀 **HashMap**: `Entry` API, double lookup'ı önler

</details>

<details>
<summary>🔽 <strong>Common Pitfalls</strong></summary>

- ⚠️ **Borrow Checker**: Collection iteration sırasında mutation
- ⚠️ **Performance**: Gereksiz `clone()` kullanımı
- ⚠️ **Memory**: Large collections için heap fragmentation

</details>

---

Bu rehberle Rust Collections konusunda **teorik bilgiyi pratikle harmanlayarak** öğreneceksiniz! 🎯🦀
