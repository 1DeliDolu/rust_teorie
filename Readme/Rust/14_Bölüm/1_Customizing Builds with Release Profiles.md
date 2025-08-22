## ⚙️ Derlemeleri Release Profilleri ile Özelleştirme (Customizing Builds with Release Profiles)

Rust’ta `release profile` (yayın profili), farklı yapılandırmalara sahip önceden tanımlanmış ve özelleştirilebilir profillerdir. Bu profiller, programcının kodu derleme (compiling) seçenekleri üzerinde daha fazla kontrol sahibi olmasına olanak tanır. Her profil diğerlerinden bağımsız olarak yapılandırılır.

Cargo’nun iki ana profili vardır:

* `dev` profili: `cargo build` komutunu çalıştırdığınızda kullanılır.
* `release` profili: `cargo build --release` komutunu çalıştırdığınızda kullanılır.

`dev` profili geliştirme için uygun varsayılanlarla tanımlanmıştır, `release` profili ise yayın derlemeleri için uygun varsayılanlara sahiptir.

Bu profil adları, derleme çıktılarında size tanıdık gelebilir:

```
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

Buradaki `dev` ve `release`, derleyicinin kullandığı farklı profillerdir.

Cargo, proje `Cargo.toml` dosyasında herhangi bir `[profile.*]` bölümü eklemediğinizde her profil için varsayılan ayarlara sahiptir. İstediğiniz herhangi bir profil için `[profile.*]` bölümleri eklediğinizde, varsayılan ayarların bir kısmını geçersiz kılabilirsiniz. Örneğin, `dev` ve `release` profilleri için `opt-level` (optimizasyon seviyesi) ayarlarının varsayılan değerleri şunlardır:

Dosya adı: `Cargo.toml`

```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`opt-level` ayarı, Rust’ın kodunuza uygulayacağı optimizasyonların sayısını kontrol eder ve 0 ile 3 arasında bir değere sahiptir. Daha fazla optimizasyon, derleme süresini uzatır. Bu nedenle geliştirme aşamasında, kodunuzu sık sık derlediğiniz için daha az optimizasyon seçmek, kodun daha yavaş çalışmasına rağmen daha hızlı derleme avantajı sağlar. Bu yüzden `dev` profilinin varsayılan `opt-level` değeri `0`’dır. Kodunuzu yayınlamaya hazır olduğunuzda ise daha uzun derleme süresini göze almak daha iyidir, çünkü yayın modunda kod yalnızca bir kez derlenecek fakat birçok kez çalıştırılacaktır. Bu nedenle `release` profilinin varsayılan `opt-level` değeri `3`tür.

Varsayılan bir ayarı, `Cargo.toml` dosyasına farklı bir değer ekleyerek geçersiz kılabilirsiniz. Örneğin geliştirme profilinde optimizasyon seviyesini `1` yapmak istiyorsak, proje `Cargo.toml` dosyamıza şu satırları ekleyebiliriz:

Dosya adı: `Cargo.toml`

```
[profile.dev]
opt-level = 1
```

Bu kod, varsayılan `0` ayarını geçersiz kılar. Artık `cargo build` çalıştırıldığında Cargo, `dev` profilinin varsayılanlarını kullanacak, ancak bizim özelleştirmemizi de uygulayacaktır. `opt-level` değerini `1` yaptığımız için Cargo, varsayılan ayardan daha fazla ama `release` modundaki kadar olmayan optimizasyonlar uygulayacaktır.

Her profil için tüm yapılandırma seçenekleri ve varsayılanların tam listesi için Cargo’nun belgelerine bakabilirsiniz.
