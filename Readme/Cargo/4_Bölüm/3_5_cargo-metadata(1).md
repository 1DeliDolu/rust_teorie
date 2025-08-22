## 📋 cargo-metadata(1)

### 🏷️ İSİM (NAME)

cargo-metadata — Mevcut paket hakkında makine tarafından okunabilir metadata (üstveri)

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo metadata [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

`stdout` üzerine, mevcut paketin çalışma alanı (workspace) üyeleri ve çözülmüş bağımlılıkları hakkında JSON çıktısı verir.

Çıktı formatı Cargo’nun gelecekteki sürümlerinde değişebilir. Kodunuzun geleceğe uyumlu olması ve beklediğiniz formatta çıktı almanızı sağlamak için `--format-version` bayrağını kullanmanız önerilir. Beklentiler hakkında daha fazla bilgi için “Uyumluluk (Compatibility)” bölümüne bakınız.

Metadata’yı okumak için Rust API sağlayan `cargo_metadata` crate’ine bakınız.

---

## 📊 ÇIKTI FORMATİ (OUTPUT FORMAT)

### 🔗 Uyumluluk (Compatibility)

Aynı çıktı formatı sürümü içinde uyumluluk korunur, ancak bazı durumlar istisna olabilir. Uyumsuzluk olarak kabul edilmeyen değişikliklere örnekler:

* Yeni alanların eklenmesi — Gerek duyulduğunda yeni alanlar eklenebilir. Bu, Cargo’nun format sürümünü sık sık artırmadan gelişmesini sağlar.
* Enum benzeri alanlara yeni değerlerin eklenmesi — Yeni alan eklemeye benzer. Metadata’nın durağanlaşmadan gelişmesini sağlar.
* Opak temsillerin değiştirilmesi — Bazı alanların iç temsilleri uygulama detayıdır. Örneğin, “Source ID” ile ilgili alanlar yalnızca paketleri veya kaynakları ayırt etmek için kullanılan opak kimliklerdir. Belirtilmedikçe bu temsillere güvenilmemelidir.

### 📑 JSON formatı

JSON çıktısı aşağıdaki formata sahiptir:

```json
{
    "packages": [
        {
            "name": "my-package",
            "version": "0.1.0",
            "id": "file:///path/to/my-package#0.1.0",
            "license": "MIT/Apache-2.0",
            "license_file": "LICENSE",
            "description": "Package description.",
            "source": null,
            "dependencies": [
                {
                    "name": "bitflags",
                    "source": "registry+https://github.com/rust-lang/crates.io-index",
                    "req": "^1.0",
                    "kind": null,
                    "rename": null,
                    "optional": false,
                    "uses_default_features": true,
                    "features": [],
                    "target": "cfg(windows)",
                    "path": "/path/to/dep",
                    "registry": null,
                    "public": false
                }
            ],
            "targets": [
                {
                    "kind": ["bin"],
                    "crate_types": ["bin"],
                    "name": "my-package",
                    "src_path": "/path/to/my-package/src/main.rs",
                    "edition": "2018",
                    "required-features": ["feat1"],
                    "doc": true,
                    "doctest": false,
                    "test": true
                }
            ],
            "features": {
                "default": ["feat1"],
                "feat1": [],
                "feat2": []
            },
            "manifest_path": "/path/to/my-package/Cargo.toml",
            "metadata": {
                "docs": {
                    "rs": {
                        "all-features": true
                    }
                }
            },
            "publish": ["crates-io"],
            "authors": ["Jane Doe <user@example.com>"],
            "categories": ["command-line-utilities"],
            "default_run": null,
            "rust_version": "1.56",
            "keywords": ["cli"],
            "readme": "README.md",
            "repository": "https://github.com/rust-lang/cargo",
            "homepage": "https://rust-lang.org",
            "documentation": "https://doc.rust-lang.org/stable/std",
            "edition": "2018",
            "links": null
        }
    ],
    "workspace_members": [
        "file:///path/to/my-package#0.1.0"
    ],
    "workspace_default_members": [
        "file:///path/to/my-package#0.1.0"
    ],
    "resolve": {
        "nodes": [
            {
                "id": "file:///path/to/my-package#0.1.0",
                "dependencies": [
                    "https://github.com/rust-lang/crates.io-index#bitflags@1.0.4"
                ],
                "deps": [
                    {
                        "name": "bitflags",
                        "pkg": "https://github.com/rust-lang/crates.io-index#bitflags@1.0.4",
                        "dep_kinds": [
                            {
                                "kind": null,
                                "target": "cfg(windows)"
                            }
                        ]
                    }
                ],
                "features": ["default"]
            }
        ],
        "root": "file:///path/to/my-package#0.1.0"
    },
    "target_directory": "/path/to/my-package/target",
    "build_directory": "/path/to/my-package/build-dir",
    "version": 1,
    "workspace_root": "/path/to/my-package",
    "metadata": {
        "docs": {
            "rs": {
                "all-features": true
            }
        }
    }
}
```

Notlar:

* `id` alanı sözdizimi için Package ID Specifications bölümüne bakınız.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 📦 Çıktı Seçenekleri (Output Options)

* `--no-deps`
  Sadece çalışma alanı üyeleri hakkında bilgi verir, bağımlılıkları almaz.

* `--format-version version`
  Kullanılacak çıktı formatı sürümünü belirtir. Şu anda yalnızca `1` geçerli değerdir.

* `--filter-platform triple`
  `resolve` çıktısını yalnızca verilen hedef `triple` için bağımlılıklarla sınırlar. Bu bayrak olmadan tüm hedefler dahil edilir.

### 🔧 Özellik Seçimi (Feature Selection)

Özellik (feature) bayrakları hangi özelliklerin etkinleştirileceğini kontrol eder. Hiçbir seçenek verilmezse varsayılan özellik etkinleştirilir.

* `-F`, `--features features`
* `--all-features`
* `--no-default-features`

### 🖥️ Görüntüleme Seçenekleri (Display Options)

* `-v`, `--verbose`
* `-q`, `--quiet`
* `--color when` (`auto`, `always`, `never`)

### 📂 Manifest Seçenekleri (Manifest Options)

* `--manifest-path path`
* `--locked`
* `--offline`
* `--frozen`
* `--lockfile-path PATH` (yalnızca nightly + `-Z unstable-options`)

### ⚙️ Ortak Seçenekler (Common Options)

* `+toolchain`
* `--config KEY=VALUE or PATH`
* `-C PATH` (yalnızca nightly + `-Z unstable-options`)
* `-h`, `--help`
* `-Z flag`

---

## 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🚦 ÇIKIŞ DURUMU (EXIT STATUS)

* `0`: Cargo başarıyla tamamlandı.
* `101`: Cargo hata ile tamamlandı.

---

## 📚 ÖRNEKLER (EXAMPLES)

Mevcut paket hakkında JSON çıktısı üretmek:

```
cargo metadata --format-version=1
```

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-pkgid(1)`, Package ID Specifications, JSON messages
