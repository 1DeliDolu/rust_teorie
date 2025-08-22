# Error Handling Projesi (Türkçe)

![rust](https://img.shields.io/badge/Rust-1.89.0-orange)

Bu proje, Rust'ın 9. Bölüm — Hata Yönetimi (Error Handling) konusunu pratikle öğrenmek için hazırlanmıştır.

Özet

- `thiserror` ile `AppError` enum'u okunaklı ve küçük tutularak tanımlandı.
- `anyhow` CLI tarafında hata zincirlerini yakalamayı ve gösterimi kolaylaştırmak için kullanıldı.
- `clap` ile CLI komutları (subcommands) eklendi: `read` ve `generate`.

Hızlı kullanım örnekleri (terminal)

1. Testleri çalıştırma

```bash
cd d:/rust_teorie/error_handling_project
cargo test
```

Beklenen test çıktısı (özet):

```
running 3 tests
test tests::test_read_first_number_not_found ... ok
test tests::test_read_first_number_parse_error ... ok
test tests::test_read_first_number_ok ... ok

test result: ok. 3 passed; 0 failed
```

2. Örnek dosya oluşturma (generate)

```bash
cargo run -- generate sample.txt 42
```

Beklenen çıktı:

```
Appended value to sample file: sample.txt
```

Not: `generate` komutu artık dosyayı üzerine yazmak yerine verilen değeri dosyanın sonuna ekler (append).
Örneğin `generate sample.txt 1` sonra `generate sample.txt 2` çalıştırılırsa, `sample.txt` içeriği sırasıyla `1` ve `2` olan iki satır içerir.

3. Dosyayı okuma (read)

```bash
cargo run -- read sample.txt
```

Beklenen çıktı:

```
First number: 42
```

Hata durumlarına örnek (örnek `test_parse.txt` içinde metin varken):

```bash
cargo run -- read test_parse.txt
```

Beklenen hata çıktısı (örnek):

```
Error: Parse error: invalid digit found in string
```

Neler öğreneceksiniz

- `Result` ve `?` ile hataları iletme
- `thiserror` ile custom error tipleri ve `From` dönüşümlerini otomatikleştirme
- `anyhow` ile CLI'da context ekleyerek hata mesajlarını zenginleştirme
- `clap` ile kullanıcı dostu CLI tasarlama (subcommand, arg parse)

Geliştirme fikirleri

- `thiserror` + `anyhow` kombinasyonunu daha büyük projelerde deneme
- `clap` ile daha fazla seçenek ekleme (`--quiet`, `--format json`)
- Hata zinciri (error chaining) ve logging entegrasyonu

Kısa görsel (ASCII)

```
	_____                 _                 _
 | ____|_   _____ _ __ | |_ ___  __ _  __| | ___ _ __
 |  _| \ \ / / _ \ '_ \| __/ _ \/ _` |/ _` |/ _ \ '__|
 | |___ \ V /  __/ | | | ||  __/ (_| | (_| |  __/ |
 |_____| \_/ \___|_| |_|\__\___|\__,_|\__,_|\___|_|

 Error handling demo — thiserror + anyhow + clap
```

Dosyalar

- `src/lib.rs` — `read_first_number` fonksiyonu ve unit testler
- `src/errors.rs` — `AppError` (thiserror)
- `src/main.rs` — CLI (clap + anyhow)

İleri adımlar

- CLI çıktılarını JSON formatında vermek (`--format json`)
- `thiserror` ile daha fazla hata varyasyonu eklemek
- `clap` ile auto-completion ve shell entegrasyonu eklemek
