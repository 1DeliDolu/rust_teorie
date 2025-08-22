## ⚖️ Reddedilebilirlik: Bir Desenin Eşleşememe Durumu (refutability: whether a pattern might fail to match)

Desenler (patterns) iki biçimde gelir: **reddedilebilir (refutable)** ve **reddedilemez (irrefutable)**.

* **Reddedilemez (irrefutable)** desenler, kendilerine verilen her değeri eşleştirebilir.
  Örneğin:

  ```rust
  let x = 5;
  ```

  Buradaki `x`, her şeyi eşleştirdiği için hiçbir zaman başarısız olamaz.

* **Reddedilebilir (refutable)** desenler, bazı değerler için eşleşmeyebilir.
  Örneğin:

  ```rust
  if let Some(x) = a_value
  ```

  Burada `a_value` değişkeni `None` olduğunda, `Some(x)` deseni eşleşmez.

---

## 📌 Nerelerde Kullanılabilir?

* **Fonksiyon parametreleri**, **let ifadeleri** ve **for döngüleri** yalnızca reddedilemez desenleri kabul eder. Çünkü eşleşme olmadığında programın yapabileceği anlamlı bir şey yoktur.
* **if let**, **while let** ve **let...else** ifadeleri hem reddedilebilir hem de reddedilemez desenleri kabul eder. Ancak derleyici, reddedilemez desenler için uyarı verir çünkü bu yapılar başarısız olma ihtimaline göre tasarlanmıştır.

Genel olarak, bu ayrım hakkında endişelenmenize gerek yoktur. Ancak bir hata mesajında gördüğünüzde ne anlama geldiğini bilmeniz gerekir. Böyle durumlarda, kodun beklenen davranışına göre ya deseni ya da deseni kullandığınız yapıyı değiştirmeniz gerekir.

---

## ❌ Reddedilebilir desenin reddedilemez yerde kullanılması

19-8 numaralı listede, `let` ifadesinde `Some(x)` reddedilebilir deseni kullanmaya çalışıyoruz. Bu kod derlenmez:

```rust
let Some(x) = some_option_value;
```

Eğer `some_option_value` `None` olursa, `Some(x)` deseni eşleşmez. Ancak `let` yalnızca reddedilemez desenleri kabul eder. Bu yüzden derleyici hata verir:

```
error[E0005]: refutable pattern in local binding
 --> src/main.rs:3:9
  |
3 |     let Some(x) = some_option_value;
  |         ^^^^^^^ pattern `None` not covered
```

---

## ✅ Çözüm: let...else kullanımı

Bunu düzeltmek için, `let` yerine `let...else` kullanabiliriz. Böylece desen eşleşmezse `else` bloğu çalışır ve program geçerli şekilde devam eder.

```rust
let Some(x) = some_option_value else {
    return;
};
```

👉 Bu kod artık geçerlidir.

---

## ⚠️ Reddedilemez desenin koşullu ifadede kullanılması

19-10 numaralı listede, `if let`’te reddedilemez bir desen (`x`) kullanıyoruz:

```rust
let x = 5 else {
    return;
};
```

Bu durumda derleyici uyarı verir:

```
warning: irrefutable `let...else` pattern
 --> src/main.rs:2:5
  |
2 |     let x = 5 else {
  |     ^^^^^^^^^
  |
  = note: this pattern will always match, so the `else` clause is useless
```

👉 Yani `else` kısmı asla çalışmaz, bu da `if let` için mantıksızdır.

---

## 🧩 match Kollarında Kullanım

* `match` kolları reddedilebilir desenler kullanmalıdır, çünkü farklı olasılıkları yakalamak gerekir.
* Ancak son kol, tüm kalan durumları kapsayacak **reddedilemez** bir desen (ör. `_`) içermelidir.
* Rust, yalnızca tek kollu bir `match` ifadesinde reddedilemez desen kullanılmasına izin verir, ama bu pek faydalı değildir çünkü daha basit bir `let` ifadesiyle aynı işi yapar.

---

Artık desenlerin nerelerde kullanılabileceğini ve **reddedilebilir (refutable)** ile **reddedilemez (irrefutable)** desenler arasındaki farkı biliyorsunuz. Şimdi desenleri oluşturmak için kullanılabilecek tüm sözdizimini inceleyeceğiz.
