## 🛠️ Geliştirme Bağımlılıkları (development dependencies)

Bazen yalnızca testler (veya örnekler, ya da benchmarklar) için bağımlılıklara ihtiyaç duyulur. Bu tür bağımlılıklar `Cargo.toml` dosyasındaki `[dev-dependencies]` bölümüne eklenir. Bu bağımlılıklar, bu pakete bağımlı olan diğer paketlere aktarılmaz.

Buna bir örnek, standart `assert_eq!` ve `assert_ne!` makrolarını genişleterek renkli fark (diff) çıktısı sağlayan **pretty\_assertions** kütüphanesidir.

---

### 📂 Örnek Dosya Yapısı

`Cargo.toml` dosyası:

```toml
# standart crate bilgileri çıkarılmıştır
[dev-dependencies]
pretty_assertions = "1"
```

`src/lib.rs` dosyası:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // yalnızca testlerde kullanılan crate. Test dışı kodda kullanılamaz.

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

---

## 📚 Ayrıca Bakınız

* Cargo dokümantasyonu: bağımlılık belirtme (specifying dependencies)
