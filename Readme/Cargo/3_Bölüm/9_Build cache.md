## 📦 Derleme Önbelleği (build cache)

Cargo, bir derlemenin çıktısını `target` dizininde saklar. Varsayılan olarak, bu dizin çalışma alanınızın (workspace) kökünde `target` adını taşır. Konumu değiştirmek için `CARGO_TARGET_DIR` ortam değişkenini (environment variable), `build.target-dir` yapılandırma değerini (config value) veya `--target-dir` komut satırı bayrağını (command-line flag) ayarlayabilirsiniz.

Dizin düzeni, belirli bir platform için derleme yapmak üzere `--target` bayrağını kullanıp kullanmadığınıza bağlıdır. Eğer `--target` belirtilmemişse, Cargo ana makine mimarisi (host architecture) için derleme yapar. Çıktı, `target` dizininin köküne gider ve her profil ayrı bir alt dizinde saklanır:

| Dizin             | Açıklama                                                                  |
| ----------------- | ------------------------------------------------------------------------- |
| `target/debug/`   | `dev` profili (profile) için çıktı içerir.                                |
| `target/release/` | `release` profili için çıktı içerir (`--release` seçeneği ile).           |
| `target/foo/`     | `foo` profili için derleme çıktısı içerir (`--profile=foo` seçeneği ile). |

Tarihsel nedenlerden dolayı, `dev` ve `test` profilleri `debug` dizininde, `release` ve `bench` profilleri `release` dizininde saklanır. Kullanıcı tanımlı profiller, profille aynı ada sahip bir dizinde saklanır.

Başka bir hedef için `--target` ile derleme yapıldığında, çıktı hedefin adıyla bir dizine yerleştirilir:

| Dizin                      | Örnek                                   |
| -------------------------- | --------------------------------------- |
| `target/<triple>/debug/`   | `target/thumbv7em-none-eabihf/debug/`   |
| `target/<triple>/release/` | `target/thumbv7em-none-eabihf/release/` |

Not: `--target` kullanılmadığında, bunun bir sonucu olarak Cargo, bağımlılıklarınızı derleme betikleri (build scripts) ve proc makrolarla (proc macros) paylaşır. `RUSTFLAGS`, her `rustc` çağrısıyla paylaşılır. `--target` bayrağı ile, derleme betikleri ve proc makrolar ana makine mimarisi için ayrı şekilde derlenir ve `RUSTFLAGS` paylaşılmaz.

Profil dizini (örneğin `debug` veya `release`) içinde, yapıtlar (artifacts) aşağıdaki dizinlere yerleştirilir:

| Dizin                    | Açıklama                                                                   |
| ------------------------ | -------------------------------------------------------------------------- |
| `target/debug/`          | Derlenen paketin çıktısını içerir (ikili dosyalar ve kütüphane hedefleri). |
| `target/debug/examples/` | Örnek hedefleri içerir.                                                    |

Bazı komutlar, çıktısını `target` dizininin üst düzeyinde ayrılmış dizinlere yerleştirir:

| Dizin             | Açıklama                                                          |
| ----------------- | ----------------------------------------------------------------- |
| `target/doc/`     | `rustdoc` belgelerini içerir (`cargo doc`).                       |
| `target/package/` | `cargo package` ve `cargo publish` komutlarının çıktısını içerir. |

Cargo ayrıca derleme süreci için gerekli birkaç başka dizin ve dosya oluşturur. Bunların düzeni Cargo’ya özeldir ve değiştirilebilir. Bu dizinlerden bazıları:

| Dizin                       | Açıklama                                                                                                 |
| --------------------------- | -------------------------------------------------------------------------------------------------------- |
| `target/debug/deps/`        | Bağımlılıklar ve diğer yapıtlar.                                                                         |
| `target/debug/incremental/` | `rustc` artımlı çıktısı (incremental output), sonraki derlemeleri hızlandırmak için kullanılan önbellek. |
| `target/debug/build/`       | Derleme betiklerinin çıktısı.                                                                            |

## 📑 Dep-info Dosyaları (dep-info files)

Her derlenmiş yapıtın (artifact) yanında `.d` uzantısına sahip bir “dep info” dosyası bulunur. Bu dosya, Makefile benzeri bir sözdizimiyle yapıtın yeniden inşa edilmesi için gerekli tüm dosya bağımlılıklarını listeler. Bunlar, dış derleme sistemleri tarafından Cargo’nun yeniden çalıştırılması gerekip gerekmediğini algılamak için kullanılmak üzere tasarlanmıştır. Dosyadaki yollar varsayılan olarak tam yol (absolute path) şeklindedir. Göreli yollar (relative paths) kullanmak için `build.dep-info-basedir` yapılandırma seçeneğine (config option) bakın.

### Örnek dep-info dosyası (`target/debug/foo.d` içinde bulunur):

```
/path/to/myproj/target/debug/foo: /path/to/myproj/src/lib.rs /path/to/myproj/src/main.rs
```

## ♻️ Paylaşılan Önbellek (shared cache)

Üçüncü taraf bir araç olan `sccache`, farklı çalışma alanları (workspaces) arasında derlenmiş bağımlılıkların paylaşılması için kullanılabilir.

`sccache` kurmak için, şu komutla yükleyin:

```
cargo install sccache
```

Ve `Cargo`’yu çağırmadan önce `RUSTC_WRAPPER` ortam değişkenini (environment variable) `sccache` olarak ayarlayın. Eğer `bash` kullanıyorsanız, `.bashrc` dosyasına şu satırı eklemek mantıklı olur:

```
export RUSTC_WRAPPER=sccache
```

Alternatif olarak, Cargo yapılandırmasında `build.rustc-wrapper` ayarını yapabilirsiniz. Daha fazla ayrıntı için `sccache` belgelerine başvurun.
