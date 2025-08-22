# 🦀 Rust Ownership Öğrenme Projesi

Bu proje, Rust'ın ownership, borrowing ve slice type kavramlarını öğrenmek için tasarlanmış interaktif bir kütüphane yönetim sistemidir.

## 🎯 Öğrenme Hedefleri

Bu projede şu konuları öğreneceksiniz:

1. **Ownership (Sahiplik)** - Rust'ın bellek yönetim sistemi
2. **Move Semantics (Taşıma Semantiği)** - Değerlerin nasıl taşındığı
3. **Borrowing (Borç Alma)** - Referanslar ve borçlanma kuralları
4. **Mutable References** - Değiştirilebilir referanslar
5. **Slice Types** - String ve array dilimlemesi

## 🚀 Projeyi Çalıştırma

```bash
cd /home/musta/rust_teorie/ownership_project
cargo run
```

## 📚 Proje Yapısı

- `src/main.rs` - Ana program ve ownership demonstrasyonları
- `src/book.rs` - Book struct'ı ve metodları
- `src/library.rs` - Library struct'ı ve collection yönetimi

## 🔍 Adım Adım Öğrenme

Program çalıştığında şu adımları takip eder:

### Adım 1: Temel Ownership
- String sahipliğinin nasıl çalıştığı
- Ownership transfer (move) kavramı

### Adım 2: Move Semantics
- Struct'ların nasıl taşındığı
- Move sonrası orijinal değişkenin kullanılamaması

### Adım 3: Borrowing
- Immutable referanslar (`&T`)
- Borçlanma kuralları

### Adım 4: Mutable References
- Mutable referanslar (`&mut T`)
- Aynı anda sadece bir mutable reference kuralı

### Adım 5: Slice Types
- String slice'lar (`&str`)
- Array slice'lar (`&[T]`)

## ⚠️ Dikkat Edilmesi Gerekenler

- Her adımda ownership kurallarının nasıl çalıştığını gözlemleyin
- Hata mesajlarını okuyun - Rust compiler çok yardımcıdır
- Kodda bulunan yorumları takip edin

## 🎓 İleri Çalışmalar

Bu temelleri öğrendikten sonra şunları deneyebilirsiniz:
- Lifetime'lar
- Smart pointers (Box, Rc, RefCell)
- Async/await ownership patterns