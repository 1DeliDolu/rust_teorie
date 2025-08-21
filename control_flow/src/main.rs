use crate::control::control_flow_example;
use crate::loops::loop_example;
use std::io;


mod control;
mod loops;

fn main() {
    println!("Please enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    control_flow_example(number);
    loop_example();
}

