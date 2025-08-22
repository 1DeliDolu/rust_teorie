use generics_project::{first_word, largest, Pair, Printable, ImportantExcerpt};

fn main() {
    let mut p = Pair::new(3, 7);
    println!("{}", p.printable());
    p.swap();
    println!("swapped: {}", p.printable());

    let nums = [10, 2, 33, 7];
    if let Some(max) = largest(&nums) {
        println!("largest: {}", max);
    }

    let s = String::from("merhaba dünya");
    println!("first word: {}", first_word(&s));

    let excerpt = ImportantExcerpt { part: "hayat kısa" };
    println!("excerpt: {}", excerpt.announce_and_return_part("Duyuru"));
}
