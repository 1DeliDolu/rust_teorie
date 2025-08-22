## 🧩 Fonksiyonel Dil Özellikleri: Yineleyiciler (iterators) ve Kapanışlar (closures)

Rust’un tasarımı birçok mevcut dilden ve teknikten ilham almıştır ve bunlardan önemli bir etki kaynağı da fonksiyonel programlamadır (functional programming). Fonksiyonel tarzda programlama genellikle fonksiyonların değer olarak kullanılması, yani onları argüman olarak geçirme, başka fonksiyonlardan döndürme, daha sonra çalıştırmak üzere değişkenlere atama gibi uygulamaları içerir.

Bu bölümde fonksiyonel programlamanın ne olup ne olmadığı tartışmasına girmeyeceğiz; bunun yerine, genellikle fonksiyonel diller olarak anılan birçok dilde bulunan özelliklere benzer Rust özelliklerini ele alacağız.

---

## 📌 Bu Bölümde Ele Alınacaklar

* **Kapanışlar (closures):** Bir değişkende saklayabileceğiniz, fonksiyona benzer yapılar
* **Yineleyiciler (iterators):** Bir dizi öğeyi işleme yöntemi
* **Kapanışlar ve yineleyicilerin kullanımı:** 12. bölümdeki G/Ç (I/O) projesini geliştirmek için nasıl kullanılabilecekleri
* **Kapanışlar ve yineleyicilerin performansı:** (küçük bir ipucu: düşündüğünüzden daha hızlılar!)

---

## 🔗 Önceki İlişkili Konular

Daha önce Rust’un desen eşleştirme (pattern matching) ve numaralandırmalar (enums) gibi fonksiyonel tarzdan etkilenen bazı özelliklerini ele almıştık. Çünkü kapanışları ve yineleyicileri öğrenmek, idiomatik (Rust’a özgü tarzda) ve hızlı Rust kodu yazmanın önemli bir parçasıdır. Bu nedenle, bu bölümün tamamını bu konulara ayıracağız.
