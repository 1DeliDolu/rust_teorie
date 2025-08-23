## 🏷️ İç İçe Döngüler ve Etiketler (nesting and labels)

İç içe (nested) döngülerle çalışırken, dış döngüleri `break` veya `continue` ile sonlandırmak mümkündür. Bu durumlarda döngülere bir **etiket (label)** verilmesi gerekir ve bu etiket `break` veya `continue` ifadesine aktarılmalıdır.

```rust
#![allow(unreachable_code, unused_labels)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
```

👉 Bu örnekte `'outer` etiketi dış döngüyü temsil ediyor. `break 'outer` ifadesi iç döngünün değil, dış döngünün sonlanmasını sağlıyor.
