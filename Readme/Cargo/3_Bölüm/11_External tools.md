## 🔧 Harici Araçlar (external tools)

Cargo’nun hedeflerinden biri, IDE’ler ve diğer derleme sistemleri gibi üçüncü taraf araçlarla kolay entegrasyondur. Entegrasyonu kolaylaştırmak için Cargo çeşitli kolaylıklar sağlar:

* Paket yapısı ve bağımlılık bilgilerini JSON olarak çıktıladığı `cargo metadata` komutu,
* Belirli bir derleme hakkında bilgi çıktıladığı `--message-format` bayrağı,
* Özel alt komut desteği.

---

## 📦 Paket Yapısı Hakkında Bilgi (information about package structure)

`cargo metadata` komutunu, paket yapısı ve bağımlılıklar hakkında bilgi almak için kullanabilirsiniz. Çıktı biçimi hakkında ayrıntılar için `cargo metadata` belgelerine bakın.

* Biçim kararlı ve sürümlüdür.
* `cargo metadata` çağırırken, ileriye dönük uyumsuzluk riskini önlemek için `--format-version` bayrağını açıkça iletmelisiniz.
* Rust kullanıyorsanız, çıktıyı ayrıştırmak için `cargo_metadata` crate’ini kullanabilirsiniz.

---

## 📑 JSON Mesajları (JSON messages)

`--message-format=json` geçirildiğinde, Cargo derleme sırasında şu bilgileri JSON olarak çıktılar:

* Derleyici hataları ve uyarıları,
* Üretilen yapıtlar (artifacts),
* Derleme betiklerinin çıktıları (örneğin yerel bağımlılıklar).

Çıktı, her satırda bir JSON nesnesi olacak şekilde `stdout`’a gider.

* `reason` alanı, mesaj türlerini ayırır.
* `package_id` alanı, pakete atıfta bulunmak için benzersiz bir kimliktir ve birçok komutta `--package` argümanı olarak kullanılabilir.

Not: `--message-format=json` yalnızca Cargo ve Rustc’nin çıktısını kontrol eder. `cargo run --message-format=json` veya prosedürel makroların çıktısı gibi diğer araçların çıktısını kontrol edemez. Bu gibi durumlarda, yalnızca `{` ile başlayan satırları JSON olarak yorumlamak geçici bir çözümdür.

`--message-format` seçeneği, JSON mesajlarının nasıl hesaplandığını ve işlendiğini değiştiren ek biçimlendirme değerleri de alabilir. Ayrıntılar için `build` komut belgelerine bakın.

Rust kullanıyorsanız, bu mesajları ayrıştırmak için yine `cargo_metadata` crate’ini kullanabilirsiniz.

⚠️ Not: `package_id`’nin bir Paket Kimliği Belirtimi (Package ID Specification) olması için MSRV: 1.77 gereklidir. Daha önce opak (opaque) idi.

---

## 📝 Derleyici Mesajları (compiler messages)

“compiler-message” mesajı, derleyiciden gelen uyarı ve hata gibi çıktıları içerir. Ayrıntılar için `rustc JSON` bölümüne bakın.

Örnek yapı:

```json
{
    "reason": "compiler-message",
    "package_id": "file:///path/to/my-package#0.1.0",
    "manifest_path": "/path/to/my-package/Cargo.toml",
    "target": {
        "kind": ["lib"],
        "crate_types": ["lib"],
        "name": "my_package",
        "src_path": "/path/to/my-package/src/lib.rs",
        "edition": "2018",
        "required-features": ["feat1"],
        "doc": true,
        "doctest": true,
        "test": true
    },
    "message": { /* ... */ }
}
```

---

## 📦 Yapıt Mesajları (artifact messages)

Her derleme adımı için bir “compiler-artifact” mesajı gönderilir:

```json
{
    "reason": "compiler-artifact",
    "package_id": "file:///path/to/my-package#0.1.0",
    "manifest_path": "/path/to/my-package/Cargo.toml",
    "target": { "kind": ["lib"], "crate_types": ["lib"], "name": "my_package", "src_path": "/path/to/my-package/src/lib.rs", "edition": "2018", "doc": true, "doctest": true, "test": true },
    "profile": { "opt_level": "0", "debuginfo": 2, "debug_assertions": true, "overflow_checks": true, "test": false },
    "features": ["feat1", "feat2"],
    "filenames": ["/path/to/my-package/target/debug/libmy_package.rlib", "/path/to/my-package/target/debug/deps/libmy_package-be9f3faac0a26ef0.rmeta"],
    "executable": null,
    "fresh": true
}
```

---

## 🛠️ Derleme Betiği Çıktısı (build script output)

“build-script-executed” mesajı, bir derleme betiğinin ayrıştırılmış çıktısını içerir. Betik çalıştırılmasa bile (önceden önbelleğe alınmış değer gösterilir) yayımlanır.

```json
{
    "reason": "build-script-executed",
    "package_id": "file:///path/to/my-package#0.1.0",
    "linked_libs": ["foo", "static=bar"],
    "linked_paths": ["/some/path", "native=/another/path"],
    "cfgs": ["cfg1", "cfg2=\"string\""],
    "env": [["SOME_KEY", "some value"], ["ANOTHER_KEY", "another value"]],
    "out_dir": "/some/path/in/target/dir"
}
```

---

## ✅ Derleme Tamamlandı (build finished)

“build-finished” mesajı, derlemenin sonunda yayımlanır:

```json
{
    "reason": "build-finished",
    "success": true
}
```

Bu mesaj, araçların JSON mesajlarını okumayı ne zaman durduracağını bilmesine yardımcı olur. Örneğin `cargo test` veya `cargo run` gibi komutlar, derleme tamamlandıktan sonra ek çıktı üretebilir. Bu mesaj, Cargo’nun artık JSON mesajı üretmeyeceğini, ancak başka çıktılar (örneğin `cargo run` ile çalıştırılan programın çıktısı) gelebileceğini bildirir.

Not: Testler için JSON çıktısına yönelik deneysel (nightly-only) destek vardır. Bu etkinleştirilirse, “build-finished” mesajından sonra testlere özgü ek JSON mesajları gelebilir.

---

## 🧩 Özel Alt Komutlar (custom subcommands)

Cargo, kendisini değiştirmeden yeni alt komutlarla genişletilebilecek şekilde tasarlanmıştır. Bu, `cargo (?<command>[^ ]+)` biçimindeki bir çağrının, `cargo-${command}` adlı harici bir aracın çağrısına dönüştürülmesiyle sağlanır. Harici aracın, kullanıcının `$PATH` dizinlerinden birinde bulunması gerekir.

* Cargo, varsayılan olarak `$CARGO_HOME/bin` içindeki araçlara `$PATH` üzerindekilere göre öncelik verir. Kullanıcılar `$CARGO_HOME/bin`’i `$PATH`’e ekleyerek bu önceliği değiştirebilir.
* Cargo, özel alt komutu çağırdığında, ilk argüman alt komutun dosya adı, ikinci argüman ise alt komutun kendisi olur. Örneğin `cargo-${command}` çağrılırken ikinci argüman `${command}` olacaktır. Ek argümanlar değiştirilmeden iletilir.
* `cargo help ${command}` komutuyla özel alt komutların yardım çıktısı gösterilebilir. Cargo, üçüncü argüman `--help` ise alt komutun yardım mesajı basacağını varsayar.

Özel alt komutlar, Cargo’ya geri çağrı yapmak için `CARGO` ortam değişkenini kullanabilir. Alternatif olarak, `cargo` crate’ine kütüphane olarak bağlanabilir, ancak bunun sakıncaları vardır:

* Cargo kütüphane olarak kararsızdır: API, kullanımdan kaldırma olmadan değişebilir.
* Bağlanan Cargo kütüphanesinin sürümü, Cargo ikili dosyasının sürümünden farklı olabilir.

Bunun yerine Cargo’yu yönlendirmek için CLI arayüzünü kullanmanız tavsiye edilir. Mevcut proje hakkında bilgi almak için `cargo metadata` komutu kullanılabilir (Rust arabirimi için `cargo_metadata` crate’i mevcuttur).
