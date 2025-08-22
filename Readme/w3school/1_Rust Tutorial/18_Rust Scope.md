## 📦 Rust Kapsam (Rust Scope)

## 📌 Kapsam (Scope)

Fonksiyonların nasıl çalıştığını artık bildiğinize göre, değişkenlerin fonksiyonların içinde ve dışında nasıl davrandığını öğrenmek önemlidir.

**Kapsam (scope)**, bir değişkenin nerede kullanılabileceğini ifade eder.

Bir değişken yalnızca oluşturulduğu blok içinde yaşar. **Blok**, süslü parantez `{ }` içindeki her şeydir.

---

## 🔹 Fonksiyon İçinde Değişken (Variable Inside a Function)

Bir fonksiyon içinde oluşturulan değişken yalnızca o fonksiyon içinde vardır:

### Örnek (Example)

```rust
fn myFunction() {
  let message = "Hello!";
  println!("{}", message);  // Burada message değişkenine erişilebilir
}

myFunction();

println!("{}", message); // Hata - message fonksiyon dışında erişilemez
```

👉 Burada `message` değişkeni yalnızca fonksiyonun içinde vardır. Dışarıda kullanmaya çalışmak hata üretir.

---

## 🔹 Blok İçinde Değişken (Variable Inside a Block)

Başka kodların içinde de (örneğin `if` veya döngülerde) bloklar oluşturabilirsiniz. Bu bloklarda tanımlanan değişkenler yalnızca o blokta geçerlidir.

### Örnek (Example)

```rust
let score = 80;

if score > 50 {
  let result = "Pass";
  println!("Result: {}", result);
}

println!("Result: {}", result); // Hata: result bu kapsamda yok
```

---

## 🔹 Aynı Kapsamda Değişkenler (Variables in the Same Scope)

Rust’ta aynı kapsamda `let` ile aynı isimde yeni bir değişken tanımlayabilirsiniz. Buna **shadowing (gölgeleme)** denir:

### Örnek (Example)

```rust
let x = 5;
let x = 10;

println!("x is: {}", x); // 10 yazdırır
```

👉 İkinci `x`, birincisini değiştirir. Artık `5` değeri erişilemez.

Bu durum bazı dillerde yasakken Rust’ta bir özelliktir. Değerleri güvenli bir şekilde dönüştürmek veya güncellemek için kullanılır.

Aynı isim farklı bir blok içinde de tekrar kullanılabilir:

### Örnek (Example)

```rust
let x = 5;

{
  let x = 10;
  println!("Inside block: {}", x);
}

println!("Outside block: {}", x);
```

👉 Burada iki `x` farklı kapsamda bulunur. İçteki `x` yalnızca blok içinde vardır. Blok dışında, orijinal değer (`5`) geçerlidir.

---

## 🎯 Kapsam Neden Önemlidir? (Why Scope Matters)

Kapsamı anlamak size şunları sağlar:

* Bir değişkenin nerede kullanılabileceğini bilmek
* İsim çakışmalarını önlemek
* Fonksiyonlar, döngüler ve koşullu ifadelerle çalışırken hataları engellemek
