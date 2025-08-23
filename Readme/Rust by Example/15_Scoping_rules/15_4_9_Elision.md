## ✂️ Örtük Yaşam Süresi (elision)

Bazı yaşam süresi (lifetime) kalıpları son derece yaygındır, bu nedenle borrow checker bunları yazım kolaylığı ve okunabilirliği artırmak için atlamanıza (omit) izin verir. Buna **elision** denir. Rust’ta elision yalnızca bu kalıplar sık kullanıldığı için vardır.

Aşağıdaki kod elision örneklerini göstermektedir. Daha kapsamlı bir açıklama için Rust kitabındaki *lifetime elision* bölümüne bakabilirsiniz:

```rust
// `elided_input` ve `annotated_input` temelde aynı imzaya sahiptir,
// çünkü `elided_input` yaşam süresi derleyici tarafından çıkarılır:
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

// Benzer şekilde, `elided_pass` ve `annotated_pass` aynı imzaya sahiptir,
// çünkü `elided_pass` için yaşam süresi derleyici tarafından örtük olarak eklenir:
fn elided_pass(x: &i32) -> &i32 { x }

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

fn main() {
    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}
```
