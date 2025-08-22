use adder::add;
use std::env;
use std::process;

fn print_usage_and_exit(program: &str) -> ! {
    eprintln!("Usage: {} <left:u64> <right:u64>", program);
    eprintln!("Example: {} 5 10", program);
    process::exit(1)
}

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| String::from("adder"));

    let left = match args.next() {
        Some(s) => match s.parse::<u64>() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Left argument is not a valid unsigned integer: {}", s);
                print_usage_and_exit(&program);
            }
        },
        None => {
            print_usage_and_exit(&program);
        }
    };

    let right = match args.next() {
        Some(s) => match s.parse::<u64>() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Right argument is not a valid unsigned integer: {}", s);
                print_usage_and_exit(&program);
            }
        },
        None => {
            print_usage_and_exit(&program);
        }
    };

    let sum = add(left, right);
    println!("The sum of {} and {} is: {}", left, right, sum);
}

