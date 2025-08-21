fn main() {
    println!("Hello, world! from main");
    another_function();
    function_with_params(5, 10);
    print_labeled_measurement(5, 'h');
    let area = calculate_area(5, 10);
    println!("The area is: {area}");
    //
     let x = plus_one(5);
     println!("The value of x is: {x}");
}


// function

fn another_function() {
    println!("This is another function!");
}

// function with parameters

fn function_with_params(param1: i32, param2: i32) {
    println!("Parameters received: {param1} and {param2}");
}

// print_labeled_measurement(5, 'h');

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// functions with return values

fn calculate_area(width: i32, height: i32) -> i32 {
    width * height
}
// function with return value
fn plus_one(x: i32) -> i32 {
    x + 1
}