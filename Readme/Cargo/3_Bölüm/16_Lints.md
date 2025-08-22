## 🔍 Lintler (lints)

Not: Cargo’nun lint sistemi kararsızdır (unstable) ve yalnızca **nightly toolchain** üzerinde kullanılabilir.

---

## ⚠️ Varsayılan Olarak Uyarı (warn-by-default)

Bu lintler varsayılan olarak **‘warn’** seviyesine ayarlanmıştır.

### 🛑 `unknown_lints`

Varsayılan: **warn**

**Ne yapar**
`[lints.cargo]` tablosundaki bilinmeyen lintleri kontrol eder.

**Neden kötü**

* Lint adı yanlış yazılmış olabilir ve bu da beklenen şekilde çalışmamasına yol açar.
* Cargo gelecekte aynı ada sahip bir lint eklerse, bilinmeyen lint bir hataya dönüşebilir.

**Örnek**

```toml
[lints.cargo]
this-lint-does-not-exist = "warn"
```

👉 Bu örnekte belirtilen `this-lint-does-not-exist` linti mevcut değildir ve uyarı üretir.
