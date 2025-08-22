## 📝 Rust Sözdizimi (Rust Syntax)

## 📌 Sözdizimi (Syntax)

İlk bölümlerde aşağıdaki kodu birkaç kez gördünüz. Şimdi bunu daha iyi anlamak için parçalayalım:

### Örnek (Example)

```rust
fn main() {
  println!("Hello World!");
}
```

## 🔍 Örneğin Açıklaması (Example explained)

* **1. satır:** `fn main()` her Rust programının başında mutlaka görülen kısımdır. `main()` bir fonksiyondur (function) ve süslü parantezler `{}` içindeki tüm kod çalıştırılır.
* **2. satır:** `println!()` bir makrodur (macro) ve ekrana metin yazdırmak için kullanılır. Örneğimizde `"Hello World!"` yazdıracaktır. Kodu bitirmek için noktalı virgül `;` kullanmayı unutmamalısınız.

## ❓ Makro Nedir? (What is a macro?)

Makro (macro), bir fonksiyon gibidir, ancak sonunda ünlem işareti `!` bulunur. Şimdilik terminoloji hakkında endişelenmeyin. Şu an için sadece şunu bilmeniz yeterlidir: Makrolar fonksiyonlara benzer (bir şeyleri çalıştırırlar), ancak her zaman fonksiyonlarla aynı kuralları takip etmezler. Makrolar hakkında daha fazlasını ileride öğreneceksiniz.

## ℹ️ Bilmek Güzel (Good to know)

Rust derleyicisi (compiler) boşlukları (white spaces) yok sayar.

Yukarıdaki kod şu şekilde de yazılabilir:

```rust
fn main(){println!("Hello World!");}
```

Ancak, birden fazla satır ve girinti (indentation) kodu daha okunabilir hale getirir.
