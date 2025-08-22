# generics_project

Küçük, öğretici Rust örnekleriyle `generics`, `traits` ve `lifetimes` konularını gösteren bir proje.

Bu crate eğitim amaçlıdır — her örnek tek bir kavramı açıklamak için sade ve okunabilir tutulmuştur.

## Hızlı başlangıç

Geliştirme ortamında projeyi çalıştırmak ve test etmek için:

```sh
cargo test
cargo run --example usage
```

## Öğrenme hedefleri

- Generic türlerin (type parameters) ne zaman ve nasıl kullanılacağını öğrenmek
- Trait tanımları ve implementasyon örnekleri görmek
- `PartialOrd`/`Copy` gibi trait bound'ları kullanarak generic fonksiyonlar yazmak
- Lifetime'ların temel kullanımını `ImportantExcerpt<'a>` örneğinde görmek

## Projedeki önemli parçalar

- `Pair<T>`: Generic bir yapı. Basit bir `Printable` benzeri trait implementasyonu ile nasıl genişletilebileceği gösteriliyor.
- `largest`: Generic bir fonksiyon; `PartialOrd` ve `Copy` trait bound'ları ile nasıl güvenli bir karşılaştırma yapılacağını gösterir.
- `first_word`: `&str` üzerinde basit parsing örneği — string dilimlerinin (slices) nasıl çalıştığını açıklar.
- `ImportantExcerpt<'a>`: Lifetime kullanan bir struct örneği; kısa bir lifetime açıklamasıyla birlikte.

Dosya yapısı (özet):

- `src/lib.rs` — crate içindeki örnek fonksiyonlar ve türler
- `src/main.rs` — (varsa) küçük runnable örnekler / demo
- `examples/usage.rs` — `cargo run --example usage` ile çalıştırılabilen örnek kullanım

## Kısa API örnekleri

Pair örneği (özet):

```rust
// Pair<T> bir çift tutar ve T herhangi bir tür olabilir
struct Pair<T> { first: T, second: T }

impl<T: std::fmt::Display> Pair<T> {
    fn print(&self) {
        println!("Pair: {} and {}", self.first, self.second);
    }
}
```

`largest` fonksiyonu (özet):

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

`ImportantExcerpt<'a>` (özet):

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

Bu örnekler, crate içindeki gerçek implementasyonların sadeleştirilmiş versiyonlarıdır; detaylar `src/` içinde bulunmaktadır.

## Yaygın tuzaklar / ipuçları

- Generic parametreleri gereksiz yere kısıtlamayın — sadece ihtiyaç duyduğunuz trait bound'ları ekleyin.
- Lifetime'lar genelde ilk başta kafa karıştırır; önce sahiplik ve borçlanma (ownership & borrowing) mantığını sağlamlaştırın.
- `Copy` trait'ini sadece gerçekten kopyalanması ucuz olan türler için kullanın.

## Önerilen alıştırmalar

1. `Pair<T>`'a `swap` metodu ekleyin.
2. `largest` fonksiyonunu `Clone` kullanarak `Copy` yerine nasıl yazabileceğinizi deneyin.
3. `ImportantExcerpt`'a bir method ekleyip, lifetime kurallarını gözlemleyin.

## Katkıda bulunma

Küçük düzeltmeler, ek açıklamalar veya daha fazla örnek için PR gönderebilirsiniz. Kod stili olarak Rust'un resmi stil rehberine (rustfmt) uyulması tercih edilir.

## Lisans

Varsa proje lisansınızı buraya ekleyin; küçük eğitim projeleri için genelde MIT veya Apache-2.0 tercih edilir.
