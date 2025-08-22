// This file has been replaced with generics/traits/lifetimes examples and tests.
pub trait Printable {
    fn printable(&self) -> String;
}

use std::fmt::Display;

/// Generic Pair tipi
pub struct Pair<T> {
    pub a: T,
    pub b: T,
}

impl<T: Display + Clone> Pair<T> {
    pub fn new(a: T, b: T) -> Self {
        Self { a, b }
    }

    pub fn swap(&mut self) {
        let tmp = self.a.clone();
        self.a = self.b.clone();
        self.b = tmp;
    }
}

impl<T: Display + Clone> Printable for Pair<T> {
    fn printable(&self) -> String {
        format!("Pair({}, {})", self.a, self.b)
    }
}

/// Slice içindeki en büyük elemanı döndürür (kopyalanabilir türler için)
pub fn largest<T: PartialOrd + Copy>(slice: &[T]) -> Option<T> {
    if slice.is_empty() {
        return None;
    }
    let mut max = slice[0];
    for &item in slice.iter().skip(1) {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

/// İlk kelimeyi döndüren basit fonksiyon (lifetime yok, &str ile çalışır)
pub fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(idx) => &s[..idx],
        None => s,
    }
}

/// Lifetime kullanan basit struct örneği
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Announcement: {}", announcement);
        self.part
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_and_first_word() {
        assert_eq!(largest(&[1, 5, 3]), Some(5));
        assert_eq!(largest(&[10i64]), Some(10));
        let s = "ilk ikinci";
        assert_eq!(first_word(s), "ilk");
    }

    #[test]
    fn pair_print_and_swap() {
        let mut p = Pair::new("a".to_string(), "b".to_string());
        assert!(p.printable().contains("a"));
        p.swap();
        assert!(p.printable().contains("b"));
    }

    #[test]
    fn excerpt_lifetime() {
        let s = String::from("hayat kısa");
        let ex = ImportantExcerpt { part: first_word(&s) };
        assert_eq!(ex.announce_and_return_part("Duyuru"), "hayat");
    }
}
