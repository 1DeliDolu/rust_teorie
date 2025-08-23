## 📦 Crates (crates)

Bir **crate (crate)**, Rust’ta bir derleme birimidir (compilation unit). `rustc some_file.rs` komutu çağrıldığında, `some_file.rs` bir crate dosyası olarak ele alınır. Eğer `some_file.rs` içinde `mod` bildirimleri varsa, bu `mod` bildirimlerinin bulunduğu yerlere ilgili modül dosyalarının içerikleri eklenir ve bundan sonra derleyici çalıştırılır. Başka bir deyişle, modüller (modules) tek başına derlenmez; yalnızca crate’ler derlenir.

Bir crate, bir **binary (ikili dosya)** ya da bir **library (kütüphane)** olarak derlenebilir. Varsayılan olarak `rustc`, bir crate’ten binary üretir. Bu davranış, `--crate-type` bayrağına `lib` değeri verilerek değiştirilebilir.
