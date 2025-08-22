## 💬 Comments / Yorumlar

Her program yorumlara (comments) ihtiyaç duyar ve Rust birkaç farklı çeşit destekler:

* Derleyici tarafından yok sayılan normal yorumlar (regular comments):

  ```rust
  // Satır yorumları (line comments) satırın sonuna kadar devam eder.
  /* Blok yorumları (block comments) kapanış sınırlayıcısına kadar devam eder. */
  ```

* HTML kütüphane belgelerine dönüştürülen dokümantasyon yorumları (doc comments):

  ```rust
  /// Sonraki öğe için kütüphane belgeleri üret.
  //! Kapsayıcı öğe için kütüphane belgeleri üret.
  ```

---

```rust
fn main() {
    // Bu bir satır yorumuna (line comment) örnektir.
    // Satırın başında iki eğik çizgi vardır.
    // Ve bundan sonra yazılan hiçbir şey derleyici tarafından okunmaz.

    // println!("Hello, world!");

    // Çalıştırın. Gördünüz mü? Şimdi iki eğik çizgiyi silin ve tekrar çalıştırın.

    /*
     * Bu başka bir yorum türü, bir blok yorumudur (block comment).
     * Genel olarak satır yorumları önerilen stildir. Ancak blok yorumları
     * kod parçalarını geçici olarak devre dışı bırakmak için son derece kullanışlıdır.
     * /* Blok yorumları /* iç içe (nested) olabilir, */ */ yani yalnızca birkaç
     * tuş vuruşuyla bu main() fonksiyonundaki her şeyi yorum satırına alabilirsiniz.
     * /*/*/* Kendiniz deneyin! */*/*/
     */

    /*
    Not: Önceki sütundaki `*` işaretleri tamamen stil içindi.
    Gerçekte gerekli değildir.
    */

    // Blok yorumlarının bir başka güçlü kullanımı: yalnızca tek bir
    // '/' karakteri ekleyerek veya kaldırarak bütün bir bloğu yorum satırı
    // haline getirebilir veya yorumdan çıkarabilirsiniz:

    /* <- tamamını yorumdan çıkarmak için ilk '/' işaretinden önce başka bir '/' ekleyin

    println!("Now");
    println!("everything");
    println!("executes!");
    // içteki satır yorumları (line comments) her iki durumda da etkilenmez

    // */

    // İfadeleri (expressions) satır yorumlarına göre blok yorumları ile
    // daha kolay değiştirebilirsiniz. Sonucu değiştirmek için yorum
    // sınırlayıcılarını silmeyi deneyin:
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
```

---

## 📚 Ayrıca bakınız (See also)

* Library documentation (Kütüphane belgeleri)
