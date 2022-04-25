// Variables hold primitive data or references to data
// Variables are immutabel by default
// Rust is block-scoped language

pub fn run() {
    let name = "Bo";

    //  mutable var
    let mut age = 37;
    // age = 38;

    println!("My name is {} and I am {}", name, age);

    //  Define constants
    //  Always caps var name and define date type
    const ID: i32 = 001;
    println!("ID {}", ID);

    //  assign multiple variables
    let (my_name, my_age) = ("Bo", 25);
    println!("{} is {}", my_name, my_age);

    //  Shadowing variables
    // declaring new varibale with same name

    let y = 10;
    let y = y * 2;
    println!("Y is {}", y);
}
