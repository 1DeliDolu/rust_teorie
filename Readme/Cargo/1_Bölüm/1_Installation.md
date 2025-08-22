## 💾 Kurulum (Installation)

### 🦀 Rust ve Cargo’yu Kurma (Install Rust and Cargo)

Cargo’yu edinmenin en kolay yolu, `rustup` kullanarak Rust’ın güncel kararlı (stable) sürümünü kurmaktır. `rustup` ile Rust’ı kurduğunuzda Cargo da beraberinde kurulacaktır.

Linux ve macOS sistemlerinde bu işlem şu şekilde yapılır:

```
curl https://sh.rustup.rs -sSf | sh
```

👉 Bu komut, bir betik dosyası indirir ve kurulumu başlatır. Her şey yolunda giderse şu mesajı göreceksiniz:

```
Rust is installed now. Great!
```

Windows’ta ise `rustup-init.exe` dosyasını indirip çalıştırın. Bu, bir konsol penceresinde kurulumu başlatacak ve başarılı olursa yukarıdaki mesajı gösterecektir.

Bundan sonra, `rustup` komutunu kullanarak Rust ve Cargo’nun `beta` veya `nightly` kanallarını da kurabilirsiniz.

Diğer kurulum seçenekleri ve bilgiler için Rust web sitesindeki **install** sayfasını ziyaret edin.

---

### ⚙️ Cargo’yu Kaynaktan Derleyip Kurma (Build and Install Cargo from Source)

Alternatif olarak, Cargo’yu kaynak koddan derleyip kurabilirsiniz.
