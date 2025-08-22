# adder

Basit bir Rust kütüphanesi ve ikili (binary) örneği: iki unsigned integer'ı toplayan `add` fonksiyonu.

Bu klasördeki önemli noktalar ve yapılmış değişiklikler:

- `src/lib.rs` — Kütüphane (library) kısmı. `pub fn add(left: u64, right: u64) -> u64` fonksiyonunu içerir ve unit test'leri buradadır.
- `src/main.rs` — CLI örneği (binary). Program iki argüman alır: iki adet `u64` sayısı, toplar ve sonucu yazdırır.

Neler düzeltildi / eklendi

- Başlangıç kodunda `src/main.rs` içinde hatalı bir `use mod;` satırı vardı; bu kaldırıldı.
- `src/main.rs` CLI argümanlarını kabul edecek şekilde güncellendi. Artık program şu şekilde çalışır:
  - `adder <left:u64> <right:u64>`
  - Örnek: `adder 5 10` çıktı: `The sum of 5 and 10 is: 15`
- `src/lib.rs` içindeki unit test'ler korunup çalıştırıldı.

Nasıl çalıştırılır

Terminalde proje dizinine gidin ve aşağıdaki komutları kullanın:

```bash
cd d:/rust_teorie/adder
# Derleme ve çalıştırma (CLI örneği)
cargo run -- 5 10

# Unit testleri çalıştırma
cargo test
```

Beklenen çıktı (örnek):

```
The sum of 5 and 10 is: 15

running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Geliştirme notları / sonraki adımlar

- CLI çıktıları veya hata mesajları isterseniz Türkçe'ye çevrilebilir.
- `adder` kütüphanesini başka projelerde (ör. `collections_project`) örnek kullanımla gösterecek küçük bir entegrasyon eklenebilir.
- İleri seviye: işlemleri BigInt destekleyecek şekilde genişletmek veya CLI'ya flag/option desteği eklemek.

Bu README, projeyi hızlıca anlamak ve yerel olarak denemek için yeterli bilgiyi sağlar.
