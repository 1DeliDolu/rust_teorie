## 🛠️ Derleme Betikleri (build scripts)

Bazı paketlerin üçüncü taraf Rust dışı kodu (örneğin C kütüphaneleri) derlemesi gerekir. Bazı paketler sistemde bulunan ya da kaynaktan derlenmesi gereken C kütüphanelerine bağlanır. Diğerleri ise derleme öncesinde ek işlevler (örneğin sözdizimi ayrıştırıcı (parser) üreteçleri) çalıştırmaya ihtiyaç duyar.

Cargo bu görevler için özelleşmiş araçların yerini almayı amaçlamaz, ancak **özel derleme betikleri (custom build scripts)** ile onlarla entegre olur. Bir paketin kök dizinine `build.rs` adlı bir dosya yerleştirmek, Cargo’nun bu betiği derleyip paketi derlemeden hemen önce çalıştırmasını sağlar.

```rust
// Özel bir build script örneği
fn main() {
    // Verilen dosya değişirse Cargo’ya betiği yeniden çalıştırmasını söyle.
    println!("cargo::rerun-if-changed=src/hello.c");
    // `cc` crate’i ile bir C dosyasını derleyip statik olarak linkle.
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
}
```

### 📌 Örnek kullanım senaryoları:

* Dahili (bundled) bir C kütüphanesi derlemek.
* Host sistemde bir C kütüphanesi bulmak.
* Bir tanımdan Rust modülü üretmek.
* Platforma özgü yapılandırmalar yapmak.

`package.build` manifest anahtarı ile build script dosyasının adı değiştirilebilir veya tamamen devre dışı bırakılabilir.

---

## 🔄 Build Script’in Yaşam Döngüsü (life cycle)

* Paket derlenmeden hemen önce Cargo, build script’i bir çalıştırılabilir dosyaya derler ve çalıştırır.
* Betik `cargo::` önekiyle stdout’a özel komutlar yazarak Cargo ile iletişim kurar.
* Betik veya bağımlılıkları değişirse yeniden derlenir.
* Varsayılan olarak, pakette herhangi bir dosya değişirse betik yeniden çalıştırılır. Ancak genellikle `cargo::rerun-if-*` komutlarıyla bu kapsam daraltılır.
* Betik `0` dışında bir çıkış kodu dönerse derleme hata ile durur.

---

## 📥 Build Script Girdileri (inputs)

* Betiğe birçok **ortam değişkeni (environment variable)** aktarılır.
* Çalışma dizini, paketin kaynak dizinidir.

---

## 📤 Build Script Çıktıları (outputs)

* Çıktılar `OUT_DIR` ortam değişkeni ile verilen dizine yazılmalıdır.
* Cargo ile iletişim, stdout’a `cargo::` önekiyle satır yazarak yapılır.
* Komutların sırası önemlidir, çünkü `rustc` ve bağlayıcıya (linker) aktarılacak argümanların sırasını etkiler.
* Normal derleme sırasında betik çıktısı gizlenir, ancak `-vv` parametresi ile görülebilir.
* Çıktılar `target/debug/build/<pkg>/output` benzeri dosyalara kaydedilir.

---

## 📑 Cargo’nun tanıdığı komutlar (özet)

* `cargo::rerun-if-changed=PATH` → Dosya değişirse betiği yeniden çalıştır.
* `cargo::rerun-if-env-changed=VAR` → Ortam değişkeni değişirse yeniden çalıştır.
* `cargo::rustc-link-arg=FLAG` → `rustc`’ye `-C link-arg` bayrağı ekle.
* `cargo::rustc-link-lib=LIB` → Belirtilen kütüphaneyi linkle.
* `cargo::rustc-link-search=[KIND=]PATH` → Arama yoluna dizin ekle.
* `cargo::rustc-flags=FLAGS` → `-l`, `-L` gibi derleyici bayraklarını ilet.
* `cargo::rustc-cfg=KEY[="VALUE"]` → Derleme zamanı `cfg` ayarlarını etkinleştir.
* `cargo::rustc-check-cfg=CHECK_CFG` → Özelleştirilmiş `cfg` değerlerini kontrol listesine ekle.
* `cargo::rustc-env=VAR=VALUE` → Derleme sırasında ortam değişkeni ayarla.
* `cargo::error=MESSAGE` → Hata mesajı göster ve derlemeyi durdur.
* `cargo::warning=MESSAGE` → Uyarı mesajı göster.
* `cargo::metadata=KEY=VALUE` → Bağımlı paketlere aktarılacak metadata üret.

---

## 📦 Build Dependencies

* Build script, `build-dependencies` bölümünde tanımlanan crate’leri kullanabilir.
* `dependencies` veya `dev-dependencies` erişilemez (çünkü henüz derlenmemiştir).
* Bağımlılık seçerken derleme süresi, lisanslar ve bakım gibi faktörlere dikkat edilmelidir.

```toml
[build-dependencies]
cc = "1.0.46"
```

---

## 🔍 Değişiklik Tespiti (change detection)

* Varsayılan: pakette herhangi bir dosya değişirse betik yeniden çalışır.
* Daha iyi kontrol için:

  * `cargo::rerun-if-changed=PATH`
  * `cargo::rerun-if-env-changed=NAME`

---

## 🔗 `links` Manifest Anahtarı

* `Cargo.toml` içinde `links = "foo"` belirtmek, paketin `libfoo` kütüphanesine bağlandığını ifade eder.
* Bu durumda:

  * Paketin bir build script’i olmalıdır.
  * Script içinde `cargo::rustc-link-lib` kullanılmalıdır.
* Aynı `links` değerine sahip birden fazla paket olamaz (çift sembol sorunlarını engellemek için).
* Build script `cargo::metadata=KEY=VALUE` çıktısı üretebilir → bağımlı paketlerde `DEP_<NAME>_<KEY>` ortam değişkenine dönüşür.

---

## 📚 `*-sys` Paketleri

* Sistem kütüphanelerine bağlanan paketler genellikle `-sys` ile biter (`foo-sys`).
* Özellikleri:

  * Yerel `libfoo` kütüphanesine bağlanır (sistemde arar veya kaynaktan kurar).
  * Yalnızca tip ve fonksiyon bildirimleri içerir (yüksek seviyeli soyutlama değil).
* Avantajları:

  * Ortak bağımlılıklar çakışmayı azaltır.
  * Metadata paylaşımıyla entegrasyonu kolaylaştırır.
  * `foo-sys` yanında genellikle `foo` adlı güvenli yüksek seviye crate bulunur.

---

## ⚙️ Build Script’leri Geçersiz Kılma (overriding)

* Bir manifest `links` içeriyorsa, build script devre dışı bırakılıp önceden tanımlanmış metadata kullanılabilir.
* Bunun için `config.toml` içine şu yapı eklenir:

```toml
[target.x86_64-unknown-linux-gnu.foo]
rustc-link-lib = ["foo"]
rustc-link-search = ["/path/to/foo"]
rustc-flags = "-L /some/path"
rustc-cfg = ['key="value"']
rustc-env = {key = "value"}
rustc-cdylib-link-arg = ["…"]
metadata_key1 = "value"
metadata_key2 = "value"
```

---

## 🧵 Jobserver

* Cargo ve `rustc`, GNU Make’in **jobserver** protokolünü kullanarak paralel işler arasında koordinasyon sağlar.
* Varsayılan iş sayısı → mantıksal CPU sayısı.
* Her build script bir iş hakkı devralır → mümkünse tek CPU kullansın.
* Daha fazla CPU gerekiyorsa `jobserver` crate kullanılmalıdır.

---

İster misin ben sana bu bölümdeki **tüm `cargo::` komutlarını** tablo halinde (*Komut*, *Açıklama*, *Ne zaman kullanılır*) özetleyeyim?
