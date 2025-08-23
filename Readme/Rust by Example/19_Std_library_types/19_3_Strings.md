## 📝 String’ler (Strings)

Rust’ta en çok kullanılan iki string türü `String` ve `&str`’dir.

* `String`, bayt vektörü (`Vec<u8>`) olarak saklanır, ancak daima geçerli bir **UTF-8** dizisi olacağı garanti edilir. `String`, öbekte (heap) ayrılır, büyüyebilir ve null ile sonlandırılmaz.
* `&str`, bir dilim (`&[u8]`) olup daima geçerli bir UTF-8 dizisine işaret eder ve bir `String`’i görüntülemek için kullanılabilir; tıpkı `&[T]`’nin `Vec<T>`’i görüntülemesi gibi.

```rust
fn main() {
    // (tüm tür açıklamaları gereksizdir)
    // Salt okunur bellekte ayrılmış bir string’e referans
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // Sözcükleri ters sırada yineleme, yeni string ayrılmaz
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Karakterleri bir vektöre kopyala, sırala ve tekrar edenleri kaldır
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // Boş ve büyüyebilir bir `String` oluştur
    let mut string = String::new();
    for c in chars {
        // String’in sonuna bir karakter ekle
        string.push(c);
        // String’in sonuna başka bir string ekle
        string.push_str(", ");
    }

    // Kırpılmış (trimmed) string, orijinal string’e bir dilimdir,
    // dolayısıyla yeni bir bellek ayrılmaz
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Öbekte bir string ayır
    let alice = String::from("I like dogs");
    // Yeni bellek ayırır ve değiştirilmiş string’i orada saklar
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}
```

👉 Daha fazla `str` / `String` metodu için `std::str` ve `std::string` modüllerine bakılabilir.

---

## 🔤 Literaller ve kaçışlar (Literals and escapes)

Rust’ta özel karakterler içeren string literalleri yazmanın birden fazla yolu vardır. Hepsi benzer bir `&str` sonucunu verir, bu yüzden yazması en uygun olan yöntem tercih edilir. Benzer şekilde, bayt string literalleri de birden çok yolla yazılabilir ve hepsi `&[u8; N]` verir.

Genelde özel karakterler ters eğik çizgi (`\`) ile kaçırılır. Bu şekilde yazdırılamayan veya klavyeden girilemeyen karakterler eklenebilir. Eğer gerçek bir ters eğik çizgi yazmak isterseniz, bir tane daha ile kaçırmalısınız: `\\`.

String veya karakter sınırlayıcıları (delimiters) bir literal içinde yer alıyorsa, onlar da kaçırılmalıdır: `"\""`, `'\''`.

```rust
fn main() {
    // Kaçışlarla baytları onaltılık değerleriyle yazabilirsiniz...
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...ya da Unicode kod noktalarıyla.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}
```

---

## 📐 Ham string literalleri (Raw string literals)

Kaçış yapılması gereken çok fazla karakter varsa veya string’i olduğu gibi yazmak daha kolaysa, **ham string literalleri** (raw string literals) kullanılabilir.

```rust
fn main() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // Ham string içinde tırnak işareti gerekirse, `#` işaretleri eklenebilir
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // Eğer string içinde "# gerekirse, daha fazla `#` kullanılabilir
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
```

---

## 💾 Bayt string’ler (Byte strings)

UTF-8 olmayan string mi istiyorsunuz? (`str` ve `String` geçerli UTF-8 olmalıdır.)
Ya da çoğunlukla metin olan bir bayt dizisi mi? **Bayt string’ler (byte strings)** burada devreye girer.

```rust
use std::str;

fn main() {
    // Bu aslında bir `&str` değildir
    let bytestring: &[u8; 21] = b"this is a byte string";

    println!("A byte string: {:?}", bytestring);

    // Bayt string’lerde bayt kaçışları kullanılabilir...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);

    // Ham bayt string’ler normal ham string’ler gibi çalışır
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // Bayt dizisini `str`’e dönüştürmek başarısız olabilir
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // Bayt string’lerin UTF-8 olması gerekmez
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" (SHIFT-JIS)

    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}
```

👉 Karakter kodlamaları (character encodings) arasında dönüşümler için `encoding` paketi kullanılabilir.

👉 String literalleri ve kaçış karakterlerinin yazılma yöntemleri hakkında daha ayrıntılı açıklama için **Rust Reference**’taki *Tokens* bölümüne bakılabilir.
