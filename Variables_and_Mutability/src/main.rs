use std::io;


fn main() {

   let guess: f32 = "251.6".parse().expect("Not a number!");
   println!("The value of guess is: {guess}");

   // numeric operations
   // ## addition
   let x = 5;
   let y = 10;
   let z = x + y;
println!("The values are: x = {x}, y = {y}, z = {z}");

   // subtraction
   let x = 5;
   let y = 10;
   let z = y - x;
 println!("The values are: x = {x}, y = {y}, z = {z}");

   // multiplication
   let x = 5;
   let y = 10;
   let z = x * y;
   println!("The values are: x = {x}, y = {y}, z = {z}");

   // division
   let x = 10;
   let y = 2;
   let z = x / y;
   println!("The values are: x = {x}, y = {y}, z = {z}");

   // remainder
   let x = 10;
   let y = 3;
   let z = x % y;
   println!("The values are: x = {x}, y = {y}, z = {z}");


   // boolean
   let x = true;
   let y = false;
   let z = x && y;
   println!("The values are: x = {x}, y = {y}, z = {z}");

   // character
let c = 'z';
let z: char = 'ℤ'; // açık tür bildirimi ile
let heart_eyed_cat = '😻';
println!("The value of c is: {c}");
println!("The value of z is: {z}");
println!("The value of heart_eyed_cat is: {heart_eyed_cat}");


// ASCII
let a = 'a';
let b = 'b';
let c = 'c';
println!("The values are: a = {a}, b = {b}, c = {c}");
println!("ASCII values: a = {}, b = {}, c = {}", a as u8, b as u8, c as u8);

// tuple type
let tupp : (i32, f64, char) = (500, 6.4, 'z');
println!("The values are: {}, {}, {}", tupp.0, tupp.1, tupp.2);

let tup = (500, 6.4, 1);

   let (_x, y, _z) = tup;

    println!("The value of y is: {y}");


     let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    println!("The values are: five_hundred = {five_hundred}, six_point_four = {six_point_four}, one = {one}");

    // ARRAY

    let arr = [1, 2, 3, 4, 5];
    println!("The values are: {}, {}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3], arr[4]);

    let months = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December",
    ];
    for (i, month) in months.iter().enumerate() {
        println!("{}. month: {}", i + 1, month);


    }

    // index access, user from console tippen
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Invalid input");

    if index < months.len() {
        println!("The month at index {} is: {}", index, months[index]);
    } else {
        println!("Invalid index");
    }

}
