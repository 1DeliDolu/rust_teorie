## 🧩 Üst Özellikler (supertraits)

Rust’ta klasik anlamda **kalıtım (inheritance)** yoktur. Ancak bir trait’i, başka bir trait’in **üst kümesi (superset)** olarak tanımlayabilirsiniz.

Örneğin:

```rust
trait Person {
    fn name(&self) -> String;
}

// Person, Student için bir üst özelliktir (supertrait).
// Student'i uygulamak için Person'u da uygulamanız gerekir.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (bilgisayar bilimi öğrencisi), hem Programmer 
// hem de Student için bir alt özelliktir (subtrait).
// CompSciStudent'i uygulamak için her iki üst özelliği de uygulamak gerekir.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {}
```
