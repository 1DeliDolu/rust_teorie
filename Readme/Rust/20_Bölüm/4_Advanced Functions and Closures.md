## ⚙️ Gelişmiş Fonksiyonlar ve Kapanışlar (advanced functions and closures)

Bu bölüm, fonksiyonlar ve kapanışlarla (closures) ilgili gelişmiş özellikleri inceler. Konular arasında **fonksiyon işaretçileri (function pointers)** ve **kapanış döndürmek (returning closures)** bulunur.

---

## 📌 Fonksiyon İşaretçileri (function pointers)

Daha önce kapanışları fonksiyonlara parametre olarak nasıl geçireceğimizi görmüştük. Aynı şeyi normal fonksiyonlarla da yapabiliriz! Bu teknik, yeni bir kapanış tanımlamak yerine halihazırda tanımlanmış bir fonksiyonu geçirmek istediğinizde faydalıdır.

Fonksiyonlar, `fn` türüne (küçük `f` ile) dönüştürülür; bu, **Fn closure trait** ile karıştırılmamalıdır. `fn` türüne **fonksiyon işaretçisi (function pointer)** denir. Fonksiyon işaretçileri sayesinde fonksiyonları başka fonksiyonlara argüman olarak geçirebiliriz.

Aşağıdaki örnekte (Listing 20-28), `add_one` adında bir fonksiyon tanımlanmıştır. `do_twice` fonksiyonu, bir `fn(i32) -> i32` türünde fonksiyon işaretçisi ve bir `i32` değer alır. Bu fonksiyon, verilen fonksiyonu iki kez çağırır ve sonuçları toplar:

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}
```

**Çıktı:**

```
The answer is: 12
```

Burada `f` parametresi, `i32` alıp `i32` döndüren bir `fn` olarak belirtilmiştir. `main` fonksiyonunda, `add_one` fonksiyon adı doğrudan `do_twice`’a geçirilebilir.

---

## 🔄 Fonksiyonlar ve Kapanışların Farkı

* `fn`, bir türdür (**trait değildir**). Bu yüzden parametre türünü doğrudan `fn` olarak yazabiliriz.
* Kapanışlar için ise `Fn`, `FnMut`, `FnOnce` gibi **trait bound**’lar kullanılır.

Fonksiyon işaretçileri bu üç kapanış trait’ini (`Fn`, `FnMut`, `FnOnce`) uygular. Yani, kapanış bekleyen fonksiyonlara da fonksiyon işaretçileri geçirilebilir. Ancak genellikle, fonksiyonların hem fonksiyonları hem de kapanışları kabul edebilmesi için generic parametre ve `Fn` trait’i tercih edilir.

**Sadece `fn` kabul etmek** istediğimiz bir durum, dış kodlarla (örneğin C dili) çalışırken ortaya çıkar. Çünkü C fonksiyonları fonksiyonları argüman olarak kabul eder, ancak kapanışları desteklemez.

---

## 🧮 map Metodu ile Kullanım Örnekleri

Standart kütüphanedeki `Iterator` trait’inin `map` metodunu ele alalım. Bu metodun argümanı olarak ister kapanış, ister fonksiyon işaretçisi kullanabiliriz.

**Kapanış ile kullanım (Listing 20-29):**

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> =
    list_of_numbers.iter().map(|i| i.to_string()).collect();
```

**Fonksiyon işaretçisi ile kullanım (Listing 20-30):**

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> =
    list_of_numbers.iter().map(ToString::to_string).collect();
```

Burada, `to_string` metodunu fonksiyon işaretçisi olarak kullandık. Birden fazla `to_string` fonksiyonu olabileceği için **tam niteleme (fully qualified syntax)** kullanmamız gerekir. Kullanılan `to_string`, `ToString` trait’inde tanımlanmıştır ve `Display` implementasyonu olan tüm türler için geçerlidir.

---

## 🏷️ Enum Başlatıcılarını Fonksiyon İşaretçisi Olarak Kullanmak

Daha önce, her `enum` varyantının aynı zamanda bir başlatıcı fonksiyon olduğunu öğrenmiştik. Bu başlatıcı fonksiyonlar da kapanış trait’lerini uygular. Yani `map` gibi kapanış alan metodlara argüman olarak geçirilebilirler (Listing 20-31):

```rust
enum Status {
    Value(u32),
    Stop,
}

let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
```

Burada, `map` metodunun her `u32` değeri için `Status::Value` başlatıcı fonksiyonu çağrılır. Bu yöntem bazı programcılar için daha okunaklıdır, bazıları ise kapanış yazmayı tercih eder. Derleyici her iki yöntemi de aynı makine koduna çevirir, bu yüzden tercih tamamen okunabilirlik üzerinedir.
## 🔄 Kapanış Döndürmek (returning closures)

Kapanışlar (closures), **trait**’ler ile temsil edilir; bu da onları doğrudan döndüremeyeceğimiz anlamına gelir. Normalde bir trait döndürmek istediğimizde, o trait’i uygulayan somut (concrete) bir tür döndürebiliriz. Ancak kapanışların derleyici tarafından oluşturulan kendilerine özgü, döndürülebilir bir somut türü yoktur.

Örneğin, kapanış bulunduğu scope’tan değerler yakalıyorsa, onu `fn` işaretçisi türü olarak döndürmemize izin verilmez.

Bunun yerine, Chapter 10’da öğrendiğimiz **`impl Trait` sözdizimini** kullanırız. Bu sayede kapanış döndürebiliriz. Örneğin (Listing 20-32):

```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
```

Bu fonksiyon, `impl Fn(i32) -> i32` türünde bir kapanış döndürür ve sorunsuz çalışır.

---

## ⚠️ Farklı Kapanış Türleri ve Hata (Listing 20-33)

Her kapanış, kendi benzersiz türüne sahiptir. Aynı imzaya (signature) sahip olsalar bile, farklı implementasyonlar farklı türler üretir. Aşağıdaki kod derlenmez:

```rust
fn main() {
    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
    move |x| x + init
}
```

**Derleyici hatası (kısaltılmış):**

```
expected opaque type, found a different opaque type
```

Bunun nedeni şudur: `impl Trait` kullanıldığında, Rust her fonksiyon için ayrı bir **opaque type (saydam olmayan tür)** oluşturur. Yani her `impl Fn(i32) -> i32` kendi başına benzersizdir. Bu yüzden aynı `Vec<T>` içine konulamazlar.

Bu davranış, Chapter 17’de gördüğümüz `async` bloklarının farklı türler üretmesine benzer.

---

## ✅ Trait Nesneleri ile Çözüm (Listing 20-34)

Bu problemi çözmek için **trait nesneleri (trait objects)** kullanabiliriz. Kapanışları `Box<dyn Fn(i32) -> i32>` içine koyarsak, aynı türü paylaşırlar:

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
```

Bu yöntemle, farklı kapanış türleri ortak bir **trait object** (`Box<dyn Fn>`) sayesinde aynı tür altında toplanır ve `Vec<T>` içine sorunsuzca konulabilir.

---

## ⏭️ Sonraki Konu

Bir sonraki bölümde **makrolar (macros)** konusuna geçiyoruz!


