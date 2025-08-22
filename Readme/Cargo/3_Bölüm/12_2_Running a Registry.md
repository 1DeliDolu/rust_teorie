## 🏃 Bir Kayıt Çalıştırma (running a registry)

En basit haliyle bir kayıt, bir **indeks** içeren bir `git` deposu ve `cargo package` tarafından oluşturulan sıkıştırılmış `.crate` dosyalarını barındıran bir sunucudan oluşabilir. Kullanıcılar bu kayıt üzerinde Cargo ile yayımlama yapamaz, ancak kapalı ortamlar için bu yeterli olabilir. İndeks formatı için bkz. **Registry Index (kayıt indeksi)**.

Yayımlamayı destekleyen tam özellikli bir kayıt ise, ek olarak Cargo’nun kullandığı API’ye uygun bir **web API servisine** ihtiyaç duyar. Bu web API, **Registry Web API (kayıt web API’si)** bölümünde açıklanmaktadır.

Bir kayıt oluşturmak ve çalıştırmak için ticari ve topluluk projeleri de mevcuttur. Uygun çözümler için şu bağlantıya bakabilirsiniz:
[https://github.com/rust-lang/cargo/wiki/Third-party-registries](https://github.com/rust-lang/cargo/wiki/Third-party-registries)

