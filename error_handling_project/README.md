# Error Handling Project

Bu proje, Rust'ın 9. Bölüm — Error Handling konusunu pratikle öğrenmek için oluşturulmuştur.

Amaç: hata türlerini (panic!, Result, custom errors) uygulamalı örneklerle göstermek.

Not: the `generate` command now appends values to the target file instead of overwriting it. Running `generate sample.txt 1` then `generate sample.txt 2` will produce a file with two lines: `1` and `2`.

Nasıl çalıştırılır:

```bash
cd d:/rust_teorie/error_handling_project
# tests
cargo test
# örnek CLI
echo "42" > example.txt
cargo run -- example.txt
```

Ne öğreneceksiniz:

- `Result` ile hataları geri döndürme ve `?` operatörü
- `From` trait ile ergonomik hata dönüşümleri
- Custom error tipleri ve `Display` implementasyonu
- Unit testlerle hata durumlarının doğrulanması

İleri geliştirme fikirleri:

- `thiserror` ve `anyhow` kütüphanelerini kullanarak hata yönetimini basitleştirme
- Daha karmaşık hata zincirlerini test etme
- CLI argümanlarını `clap` ile geliştirme
