## 📋 cargo-report(1)

### 🏷️ İSİM (NAME)

cargo-report — Çeşitli türlerde raporlar oluşturur ve görüntüler

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo report type [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Belirtilen türde bir rapor görüntüler — şu anda yalnızca `future-incompat` desteklenmektedir.

---

### ⚙️ SEÇENEKLER (OPTIONS)

`--id id`
Belirtilen Cargo tarafından üretilen `id` ile raporu gösterir.

`-p spec…`
`--package spec…`
Yalnızca belirtilen paket için raporu görüntüler.

---

### 📚 ÖRNEKLER (EXAMPLES)

En son `future-incompat` raporunu görüntüle:

```
cargo report future-incompat
```

👉 Bu komut, en güncel gelecekteki uyumsuzluk raporunu listeler.

Belirli bir paket için en son `future-incompat` raporunu görüntüle:

```
cargo report future-incompat --package my-dep:0.0.1
```

👉 Bu komut, yalnızca `my-dep:0.0.1` paketi için gelecekteki uyumsuzluk raporunu gösterir.

---

### 🔗 BAKINIZ (SEE ALSO)

Future incompat report

`cargo(1)`
