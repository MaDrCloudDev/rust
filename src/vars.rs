// Variables hold primitive data or references to data
// Variables are immutable by default (can't reassign by default)
// Rust is a block-scoped language

pub fn run() {
    let name = "Matt";
    let mut age = 36;

    println!("My name is {} and I'm {}", name, age);

    age = 33;

    println!("My name is {} and I'm {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Matt", 37);
    println!("{} is {}", my_name, my_age);
}
