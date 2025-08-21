

pub fn loop_example() {
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break;
        }
        println!("Count is: {}", count);
    }
}   