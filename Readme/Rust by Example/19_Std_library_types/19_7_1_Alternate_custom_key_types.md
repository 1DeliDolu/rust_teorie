## 🔑 Alternatif / özel anahtar türleri (Alternate/custom key types)

`HashMap`’te anahtar olarak kullanılabilecek herhangi bir tür, `Eq` ve `Hash` trait’lerini uygulamalıdır. Bu şunları kapsar:

* `bool` (pek kullanışlı değil çünkü yalnızca iki olası anahtar vardır)
* `int`, `uint` ve tüm varyasyonları
* `String` ve `&str` (ipucu: `String` anahtarlı bir HashMap oluşturup `.get()` çağrısını `&str` ile yapabilirsiniz)

Not: `f32` ve `f64` türleri `Hash` uygulamaz. Bunun sebebi, kayan nokta (floating-point) hassasiyet hatalarının `HashMap` anahtarı olarak kullanıldığında ciddi hatalara yol açabilmesidir.

Tüm koleksiyon sınıfları (örneğin `Vec<T>`), içerdiği tür (`T`) `Eq` ve `Hash` uygularsa kendisi de `Eq` ve `Hash` uygular.

Bir özel tür için `Eq` ve `Hash` uygulamak kolaydır; tek satır yeterlidir:

```rust
#[derive(PartialEq, Eq, Hash)]
```

Derleyici gerisini halleder. Daha fazla denetim isterseniz, `Eq` ve/veya `Hash` trait’lerini kendiniz de elle uygulayabilirsiniz.

---

### 🧑 Basit bir kullanıcı giriş sistemi örneği

```rust
use std::collections::HashMap;

// Eq için PartialEq de türetilmelidir.
#[derive(PartialEq, Eq, Hash)]
struct Account<'a>{
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a>{
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>,
        username: &'a str, password: &'a str){
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon...");

    let logon = Account {
        username,
        password,
    };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful logon!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        },
        _ => println!("Login failed!"),
    }
}

fn main(){
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "j.everyman", "psasword123");

    try_logon(&accounts, "j.everyman", "password123");
}
```

👉 Bu örnekte `Account` struct’ı `HashMap`’te anahtar olarak kullanılmaktadır. `#[derive(PartialEq, Eq, Hash)]` eklenerek `HashMap`’in özel türlerle sorunsuz çalışması sağlanır.
