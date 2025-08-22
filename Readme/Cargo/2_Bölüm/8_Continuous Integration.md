## 🔄 Sürekli Entegrasyon (Continuous Integration)

### 🚀 Başlarken (Getting Started)

Temel bir CI (Continuous Integration) yapılandırması projelerinizi derler ve test eder.

---

### ⚙️ GitHub Actions

Bir paketi **GitHub Actions** üzerinde test etmek için örnek `.github/workflows/ci.yml` dosyası:

```yaml
name: Cargo Build & Test

on:
  push:
  pull_request:

env: 
  CARGO_TERM_COLOR: always

jobs:
  build_and_test:
    name: Rust project - latest
    runs-on: ubuntu-latest
    strategy:
      matrix:
        toolchain:
          - stable
          - beta
          - nightly
    steps:
      - uses: actions/checkout@v4
      - run: rustup update ${{ matrix.toolchain }} && rustup default ${{ matrix.toolchain }}
      - run: cargo build --verbose
      - run: cargo test --verbose
```

👉 Bu yapılandırma üç sürüm kanalında test eder: `stable`, `beta`, `nightly`. Herhangi birinde hata olması tüm işin başarısız olmasına sebep olur.

GitHub arayüzünden *Actions > new workflow* seçip Rust şablonunu ekleyerek varsayılan yapılandırmayı da kullanabilirsiniz.

---

### ⚙️ GitLab CI

Bir paketi **GitLab CI** üzerinde test etmek için örnek `.gitlab-ci.yml`:

```yaml
stages:
  - build

rust-latest:
  stage: build
  image: rust:latest
  script:
    - cargo build --verbose
    - cargo test --verbose

rust-nightly:
  stage: build
  image: rustlang/rust:nightly
  script:
    - cargo build --verbose
    - cargo test --verbose
  allow_failure: true
```

👉 Bu yapılandırma `stable` ve `nightly` kanallarında test eder. `nightly` başarısız olsa bile genel iş başarısız sayılmaz.

---

### ⚙️ builds.sr.ht

**sr.ht** üzerinde test etmek için örnek `.build.yml`:

```yaml
image: archlinux
packages:
  - rustup
sources:
  - <your repo>
tasks:
  - setup: |
      rustup toolchain install nightly stable
      cd <your project>/
      rustup run stable cargo fetch
  - stable: |
      rustup default stable
      cd <your project>/
      cargo build --verbose
      cargo test --verbose
  - nightly: |
      rustup default nightly
      cd <your project>/
      cargo build --verbose ||:
      cargo test --verbose  ||:
  - docs: |
      cd <your project>/
      rustup run stable cargo doc --no-deps
      rustup run nightly cargo doc --no-deps ||:
```

👉 `stable` ve `nightly` kanallarında test ve dokümantasyon oluşturur. `nightly` başarısız olsa bile iş başarısız sayılmaz.

---

### ⚙️ CircleCI

**CircleCI** üzerinde test etmek için örnek `.circleci/config.yml`:

```yaml
version: 2.1
jobs:
  build:
    docker:
      # check https://circleci.com/developer/images/image/cimg/rust#image-tags for latest
      - image: cimg/rust:1.77.2
    steps:
      - checkout
      - run: cargo test
```

👉 Daha karmaşık pipeline’lar için *CircleCI Configuration Reference* belgesine göz atabilirsiniz.

---

## ✅ En Güncel Bağımlılıkların Doğrulanması (Verifying Latest Dependencies)

`Cargo.toml` dosyasında belirtilen bağımlılıklar genellikle bir sürüm aralığına karşılık gelir. Tüm kombinasyonları test etmek zor olacağından, en azından en güncel sürümleri test etmek önemlidir.

Dikkate alınacak noktalar:

* Yerel geliştirme/CI üzerinde dış etkenleri en aza indirmek
* Yeni bağımlılıkların yayınlanma sıklığı
* Projenin kabul ettiği risk seviyesi
* CI maliyetleri (özellikle paralel iş limitleri)

Olası çözümler:

* `Cargo.lock` dosyasını sürüm kontrolüne dahil etmemek
* En güncel bağımlılıkları test eden ama hata olsa bile devam eden bir CI işi eklemek
* Zamanlanmış (scheduled) CI işlerinde en güncel bağımlılıkları test etmek
* **Dependabot** veya **RenovateBot** gibi araçlarla bağımlılık güncellemelerini otomatik PR’lar aracılığıyla yönetmek

GitHub Actions üzerinde en güncel bağımlılıkları doğrulamak için örnek iş:

```yaml
jobs:
  latest_deps:
    name: Latest Dependencies
    runs-on: ubuntu-latest
    continue-on-error: true
    env:
      CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS: allow
    steps:
      - uses: actions/checkout@v4
      - run: rustup update stable && rustup default stable
      - run: cargo update --verbose
      - run: cargo build --verbose
      - run: cargo test --verbose
```

👉 Burada `CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=allow` değişkeni, seçilen bağımlılıkların Rust sürümü nedeniyle kısıtlanmamasını sağlar.

---

## 🦀 rust-version Doğrulama (Verifying rust-version)

Bir paket `rust-version` alanını belirtiyorsa, bu alanın doğruluğunu kontrol etmek önemlidir.

Bunun için yardımcı olabilecek üçüncü taraf araçlar:

* `cargo-msrv`
* `cargo-hack`

GitHub Actions üzerinde örnek doğrulama:

```yaml
jobs:
  msrv:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: taiki-e/install-action@cargo-hack
    - run: cargo hack check --rust-version --workspace --all-targets --ignore-private
```

👉 Bu yöntem tek bir platformda çalıştırılır (projelerin çoğu platformdan bağımsızdır) ve `cargo check` kullanır. Böylece API uyumluluğu kontrol edilir, davranış testinden çok erişilebilirlik doğrulanır. Yayınlanmamış paketler atlanır çünkü `rust-version` doğrulaması esas olarak kayıttan tüketilen paketler için önemlidir.
