## 📊 Gelecekteki Uyumsuzluk Raporu (future incompat report)

Cargo, tüm bağımlılıklarda gelecekte uyumsuz olabilecek uyarıları denetler. Bunlar, gelecekte `rustc` derleyicisinin sürümlerinde **hata (error)** haline gelebilecek ve bağımlılığın derlenmesini engelleyebilecek uyarılardır. Eğer herhangi bir uyarı bulunursa, küçük bir bildirim gösterilir ve tam raporun nasıl görüntüleneceğine dair talimatlar sağlanır.

Örneğin, bir derlemenin sonunda aşağıdakine benzer bir şey görebilirsiniz:

```
warning: the following packages contain code that will be rejected by a future
         version of Rust: rental v0.5.5
note: to see what the problems were, use the option `--future-incompat-report`,
      or run `cargo report future-incompatibilities --id 1`
```

Tam rapor, `cargo report future-incompatibilities --id ID` komutu ile veya derlemeyi `--future-incompat-report` bayrağı ile yeniden çalıştırarak görüntülenebilir. Geliştirici daha sonra bağımlılıklarını sorunun düzeltildiği bir sürüme güncellemeli veya bağımlılıkların geliştiricileriyle birlikte çalışarak sorunu çözmelidir.

---

## ⚙️ Yapılandırma (configuration)

Bu özellik `.cargo/config.toml` dosyasındaki `[future-incompat-report]` bölümü aracılığıyla yapılandırılabilir. Şu anda desteklenen seçenekler şunlardır:

```toml
[future-incompat-report]
frequency = "always"
```

`frequency` için desteklenen değerler `"always"` ve `"never"` olup, bunlar `cargo build` / `cargo check` komutlarının sonunda bir mesajın gösterilip gösterilmeyeceğini kontrol eder.

