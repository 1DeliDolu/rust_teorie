## 🔑 Git Kimlik Doğrulama (Git Authentication)

Cargo, git bağımlılıkları (git dependencies) ve kayıt defterleri (registries) kullanıldığında bazı kimlik doğrulama biçimlerini destekler. Bu ek bölüm, Cargo ile çalışan bir şekilde git kimlik doğrulamasını ayarlamak için bazı bilgiler içerir.

Başka kimlik doğrulama yöntemlerine ihtiyacınız varsa, `net.git-fetch-with-cli` yapılandırma değeri ayarlanarak Cargo’nun dahili desteği kullanmak yerine git yürütülebilir dosyasını çalıştırarak uzak depoları indirmesi sağlanabilir. Bu, `CARGO_NET_GIT_FETCH_WITH_CLI=true` ortam değişkeni (environment variable) ile etkinleştirilebilir.

Not: Cargo, herkese açık git bağımlılıkları için kimlik doğrulaması gerektirmez. Bu bağlamda bir kimlik doğrulama hatası görürseniz, URL’nin doğru olduğundan emin olun.

---

### 🌐 HTTPS Kimlik Doğrulama (HTTPS Authentication)

HTTPS kimlik doğrulama, `credential.helper` mekanizmasını gerektirir. Birden fazla kimlik doğrulama yardımcısı (credential helper) vardır ve kullanılacak olan, küresel git yapılandırma dosyasında belirtilir.

`~/.gitconfig`

```
[credential]
helper = store
```

Cargo parola sormaz, bu nedenle çoğu yardımcı için Cargo’yu çalıştırmadan önce yardımcıya ilk kullanıcı adı/parolayı vermeniz gerekir. Bunun bir yolu, özel git deposunu `git clone` ile klonlayarak kullanıcı adı/parolayı girmektir.

İpucu:

* macOS kullanıcıları `osxkeychain` yardımcısını kullanmayı düşünebilir.
* Windows kullanıcıları `GCM` yardımcısını kullanabilir.

Not: Windows kullanıcılarının `sh` kabuğunun (shell) `PATH` içinde olduğundan emin olması gerekir. Bu genellikle Git for Windows kurulumu ile birlikte gelir.

---

### 🔐 SSH Kimlik Doğrulama (SSH Authentication)

SSH kimlik doğrulama, SSH anahtarını almak için `ssh-agent`’ın çalışıyor olmasını gerektirir. Gerekli ortam değişkenlerinin (çoğu Unix-benzeri sistemde `SSH_AUTH_SOCK`) ayarlı olduğundan ve doğru anahtarların `ssh-add` ile eklendiğinden emin olun.

Windows, `Pageant` (PuTTY’nin bir parçası) veya `ssh-agent` kullanabilir. `ssh-agent` kullanmak için Cargo’nun Windows ile birlikte dağıtılan OpenSSH’yi kullanması gerekir; çünkü Cargo, MinGW veya Cygwin tarafından kullanılan taklit edilmiş Unix etki alanı soketlerini desteklemez. Daha fazla bilgi Microsoft’un kurulum belgelerinde bulunabilir. Anahtar yönetimi sayfasında ise `ssh-agent` başlatma ve anahtar ekleme hakkında talimatlar vardır.

Notlar:

* Cargo, `git@example.com:user/repo.git` gibi git’in kısayol SSH URL’lerini desteklemez. Bunun yerine tam SSH URL’si kullanın: `ssh://git@example.com/user/repo.git`
* SSH yapılandırma dosyaları (örneğin OpenSSH’nin `~/.ssh/config` dosyası) Cargo’nun dahili SSH kütüphanesi tarafından kullanılmaz. Daha gelişmiş gereksinimler için `net.git-fetch-with-cli` kullanılmalıdır.

---

### 🗝️ SSH Known Hosts

Bir SSH ana bilgisayarına bağlanırken, Cargo ana bilgisayarın kimliğini “known hosts” listesiyle doğrulamalıdır. Bu, anahtarların (host keys) bulunduğu listedir. Cargo, bilinen ana bilgisayarları standart konumlarda bulunan OpenSSH tarzı `known_hosts` dosyalarında arar:

* `~/.ssh/known_hosts` (ev dizininde)
* `/etc/ssh/ssh_known_hosts` (Unix-benzeri sistemlerde)
* `%PROGRAMDATA%\ssh\ssh_known_hosts` (Windows’ta)

Bu dosyalar hakkında daha fazla bilgi `sshd` man sayfasında bulunabilir. Alternatif olarak, anahtarlar Cargo yapılandırma dosyasında `net.ssh.known-hosts` ile tanımlanabilir.

Bir SSH ana bilgisayarına, bilinen ana bilgisayarlar yapılandırılmadan önce bağlanıldığında, Cargo size anahtarın nasıl ekleneceğini açıklayan bir hata mesajı gösterecektir. Bu mesajda ayrıca, anahtarın daha kolay görsel olarak doğrulanmasını sağlayan bir “fingerprint” (parmak izi) bulunur. Sunucu yöneticisi, genel anahtara (`/etc/ssh/ssh_host_ecdsa_key.pub`) karşı `ssh-keygen` çalıştırarak bu parmak izini alabilir. Tanınmış siteler parmak izlerini web üzerinden yayımlayabilir; örneğin GitHub kendi parmak izlerini [https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/githubs-ssh-key-fingerprints](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/githubs-ssh-key-fingerprints) adresinde paylaşmaktadır.

Cargo, `github.com` için ana bilgisayar anahtarlarını yerleşik olarak içerir. Bunlar değişirse, yeni anahtarları yapılandırmaya veya `known_hosts` dosyasına ekleyebilirsiniz.

Not: Cargo, `known_hosts` dosyalarındaki `@cert-authority` veya `@revoked` işaretlerini desteklemez. Bu işlevi kullanmak için `net.git-fetch-with-cli` kullanılmalıdır. Ayrıca, Cargo’nun SSH istemcisi beklendiğiniz gibi çalışmıyorsa bu yöntem önerilir.
