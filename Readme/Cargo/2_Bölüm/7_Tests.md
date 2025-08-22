## 🧪 Testler (Tests)

Cargo, testlerinizi `cargo test` komutu ile çalıştırabilir. Cargo testleri iki yerde arar:

1. `src` dizinindeki dosyalarınızda
2. `tests/` dizininde

* `src` içindeki testler birim testleri (**unit tests**) ve dokümantasyon testleri (**documentation tests**) olmalıdır.
* `tests/` içindekiler ise entegrasyon tarzı (**integration tests**) testlerdir. Bu nedenle `tests` dizinindeki dosyalarda crate’inizi içe aktarmanız gerekir.

---

### 📌 Örnek: Test Çalıştırmak

Testi olmayan bir pakette `cargo test` çıktısı şu şekilde olur:

```
$ cargo test
   Compiling regex v1.5.0 (https://github.com/rust-lang/regex.git#9f9f693)
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
     Running target/test/hello_world-9c2b65bbb79eabce

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

👉 Eğer paketinizde testler olsaydı, çalıştırılan test sayısını ve sonuçlarını görecektiniz.

---

### 🎯 Belirli Bir Test Çalıştırmak

Bir filtre geçirerek belirli bir testi çalıştırabilirsiniz:

```
$ cargo test foo
```

👉 Bu komut adı içinde `foo` geçen tüm testleri çalıştırır.

---

### ✅ Ek Kontroller

`cargo test` ayrıca ekstra kontroller de yapar:

* Yazdığınız tüm **örnekleri (examples)** derleyerek geçerli olduklarını doğrular.
* **Dokümantasyon testlerini** çalıştırarak açıklama satırlarınızdaki kod örneklerinin derlenebilir olduğunu kontrol eder.

---

### 📖 Daha Fazla Bilgi

Test yazma ve düzenleme hakkında genel bir bakış için Rust dokümantasyonundaki **testing guide** bölümüne bakabilirsiniz.
Farklı test stilleri hakkında ayrıntılar için **Cargo Targets: Tests** bölümünü inceleyebilirsiniz.
