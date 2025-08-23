## 🧵 İş parçacıkları (Threads)

Rust, `spawn` fonksiyonu aracılığıyla yerel işletim sistemi (operating system) iş parçacıklarını başlatmak için bir mekanizma sağlar; bu fonksiyonun argümanı, taşınan bir kapanıştır (moving closure, closure).

```rust
use std::thread;

const NTHREADS: u32 = 10;

// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
```

Bu iş parçacıkları, işletim sistemi tarafından zamanlanacaktır.
