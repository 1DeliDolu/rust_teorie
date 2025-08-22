# generics_project

Modern, öğretici örneklerle Rust generics, traits ve lifetimes konularını gösteren küçük bir kütüphane.

Bu proje `Readme/Rust/10_Bölüm` içeriğine paralel olarak hazırlanmıştır ve aşağıdaki konuları hedefler:

- Generic veri tipleri ve fonksiyonlar
- Trait tanımları ve trait bound kullanımı
- Lifetime'lar ve referans doğrulamaları

## Öne çıkanlar

- `Pair<T>`: generic bir yapı ve `Printable` trait implementasyonu
- `largest<T>`: generic fonksiyon (PartialOrd + Copy)
- `first_word`: &str ile basit parsing örneği
- `ImportantExcerpt<'a>`: lifetime kullanan struct örneği
- Detaylı örnekler: `examples/generics.rs`, `examples/traits.rs`, `examples/lifetimes.rs`

## Başlarken

Projeyi çalıştırmak ve test etmek için:

```bash
cd /mnt/d/rust_teorie/generics_project
cargo test
cargo run --example generics    # Generic types örnekleri
cargo run --example traits      # Traits örnekleri
cargo run --example lifetimes   # Lifetimes örnekleri
cargo run --example usage       # Genel kullanım örneği
```

## Detaylı Örnekler

### 1. Generic Types (`examples/generics.rs`)

Generic türlerin nasıl çalıştığını gösteren kapsamlı örnek:

```bash
cargo run --example generics
```

Bu örnek şunları gösterir:

- `Pair<T>` struct'ı farklı türlerle (i32, String)
- `largest<T>` fonksiyonu sayılar ve karakterlerle
- `first_word` fonksiyonu string parsing için
- Tüm örnekler assertion'larla test edilir

### 2. Traits (`examples/traits.rs`)

Trait tanımları ve kullanım örnekleri:

```bash
cargo run --example traits
```

Bu örnek şunları gösterir:

- `Printable` trait implementasyonu
- Generic fonksiyonlarda trait bound kullanımı
- `where` clause alternatif syntax
- Farklı türlerle trait kullanımı

### 3. Lifetimes (`examples/lifetimes.rs`)

Lifetime'ların nasıl çalıştığını gösteren örnekler:

```bash
cargo run --example lifetimes
```

Bu örnek şunları gösterir:

- `ImportantExcerpt<'a>` struct kullanımı
- Custom lifetime fonksiyonları
- Scope ve lifetime ilişkisi
- Pratik lifetime senaryoları

## API Referansı

### `Pair<T>`

```rust
pub struct Pair<T> {
    pub a: T,
    pub b: T,
}

impl<T: Display + Clone> Pair<T> {
    pub fn new(a: T, b: T) -> Self
    pub fn swap(&mut self)
}
```

### `largest<T>`

```rust
pub fn largest<T: PartialOrd + Copy>(slice: &[T]) -> Option<T>
```

### `first_word`

```rust
pub fn first_word(s: &str) -> &str
```

### `ImportantExcerpt<'a>`

```rust
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}
```

## Testler

Projede birim testleri `src/lib.rs` içinde yer alır:

```bash
cargo test
```

Ayrıca örnekler de CI'da otomatik test edilir ve assertion'larla doğrulanır.

## CI / Kalite Kontrolleri

Bu proje kapsamlı CI içerir (`.github/workflows/ci.yml`):

- Format check (`cargo fmt`)
- Linting (`cargo clippy`)
- Build ve test (stable, beta, nightly)
- Example testing (tüm örneklerin çalıştırılması)
- Latest dependencies test
- MSRV (Minimum Supported Rust Version) check

## Öğrenme Kaynakları

Bu proje `d:\rust_teorie\Readme\Rust\10_Bölüm` içindeki dokümanlara uygun örnekler içerir:

- `0_Generic Types, Traits, and Lifetimes.md`
- `1_Generic Data Types.md`
- `2_Traits Defining Shared Behavior.md`
- `3_Validating References with Lifetimes.md`

## Geliştirme

- Kod stilini korumak için `cargo fmt` kullanın
- Linting için `cargo clippy` çalıştırın
- Yeni örnekler eklerken assertion'lar eklemeyi unutmayın
