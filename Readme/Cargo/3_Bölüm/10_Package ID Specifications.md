## 📦 Paket Kimliği Belirtimleri (package ID specifications)

Cargo’nun alt komutları, güncelleme, temizleme, derleme gibi çeşitli işlemler için sık sık bağımlılık grafiği (dependency graph) içindeki belirli bir pakete atıfta bulunmak zorundadır. Bu sorunu çözmek için Cargo, **Paket Kimliği Belirtimleri**ni (Package ID Specifications) destekler. Bir belirtim (specification), paketler grafiğinde tek bir pakete benzersiz şekilde atıfta bulunmak için kullanılan bir dizedir (string).

Belirtim tam nitelikli olabilir, örneğin:
`https://github.com/rust-lang/crates.io-index#regex@1.4.3`
veya kısaltılmış olabilir, örneğin:
`regex`

Kısaltılmış biçim, bağımlılık grafiğinde tek bir paketi benzersiz olarak tanımladığı sürece kullanılabilir. Eğer belirsizlik varsa, benzersizliği sağlamak için ek niteleyiciler eklenebilir. Örneğin, grafikte `regex` paketinin iki sürümü varsa, sürümle birlikte nitelemek gerekir:
`regex@1.4.3`

---

## 📜 Belirtim Dilbilgisi (specification grammar)

Bir **Paket Kimliği Belirtimi** (Package ID Specification) için resmi dilbilgisi şöyledir:

```
spec := pkgname |
        [ kind "+" ] proto "://" hostname-and-path [ "?" query] [ "#" ( pkgname | semver ) ]
query = ( "branch" | "tag" | "rev" ) "=" ref
pkgname := name [ ("@" | ":" ) semver ]
semver := digits [ "." digits [ "." digits [ "-" prerelease ] [ "+" build ]]]
```

```
kind = "registry" | "git" | "path"
proto := "http" | "git" | "file" | ...
```

Burada köşeli parantezler içeriğin isteğe bağlı olduğunu belirtir.

* **URL biçimi**, git bağımlılıkları (git dependencies) için veya farklı kayıt defterlerinden (registries) gelen paketleri ayırt etmek için kullanılabilir.

---

## 📊 Örnek Belirtimler (example specifications)

Aşağıdakiler crates.io üzerindeki `regex` paketine atıflardır:

| Spec                                                                | Ad (Name) | Sürüm (Version) |
| ------------------------------------------------------------------- | --------- | --------------- |
| `regex`                                                             | regex     | \*              |
| `regex@1.4`                                                         | regex     | 1.4.\*          |
| `regex@1.4.3`                                                       | regex     | 1.4.3           |
| `https://github.com/rust-lang/crates.io-index#regex`                | regex     | \*              |
| `https://github.com/rust-lang/crates.io-index#regex@1.4.3`          | regex     | 1.4.3           |
| `registry+https://github.com/rust-lang/crates.io-index#regex@1.4.3` | regex     | 1.4.3           |

---

Aşağıdakiler çeşitli git bağımlılıkları (git dependencies) için bazı belirtim örnekleridir:

| Spec                                                                  | Ad (Name)      | Sürüm (Version) |
| --------------------------------------------------------------------- | -------------- | --------------- |
| `https://github.com/rust-lang/cargo#0.52.0`                           | cargo          | 0.52.0          |
| `https://github.com/rust-lang/cargo#cargo-platform@0.1.2`             | cargo-platform | 0.1.2           |
| `ssh://git@github.com/rust-lang/regex.git#regex@1.4.3`                | regex          | 1.4.3           |
| `git+ssh://git@github.com/rust-lang/regex.git#regex@1.4.3`            | regex          | 1.4.3           |
| `git+ssh://git@github.com/rust-lang/regex.git?branch=dev#regex@1.4.3` | regex          | 1.4.3           |

---

Yerel dosya sistemindeki (local filesystem) paketler `file://` URL’leriyle referans alınabilir:

| Spec                                        | Ad (Name) | Sürüm (Version) |
| ------------------------------------------- | --------- | --------------- |
| `file:///path/to/my/project/foo`            | foo       | \*              |
| `file:///path/to/my/project/foo#1.1.8`      | foo       | 1.1.8           |
| `path+file:///path/to/my/project/foo#1.1.8` | foo       | 1.1.8           |

---

## ✂️ Belirtimlerin Kısalığı (brevity of specifications)

Amaç, bağımlılık grafiğinde paketlere atıfta bulunmak için hem **kısa** hem de **ayrıntılı** sözdizimlerini (syntaxes) mümkün kılmaktır. Belirsiz referanslar bir veya daha fazla pakete işaret edebilir. Çoğu komut, aynı belirtimle birden fazla pakete atıfta bulunulabiliyorsa hata üretir.
