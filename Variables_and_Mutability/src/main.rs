fn main() {

    //let x = 5 
    let mut x   = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants 

    const THREE_HOURS_IN_SECONDS: i32 = 3 * 60 * 60;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 5;
    println!("The value of x is before shadowing: {x}");
    let x = x + 1;
    println!("The value of x is after shadowing: {x}"); 

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The number of spaces is: {spaces}");

}
