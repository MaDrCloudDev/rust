// Conditionals - used to check the condition of something and act on the result (no ternary operator in rust)

pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;

    // If/else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartend: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartend: Sorry, you must be 21!");
    } else {
        println!("Bartend: I'll need your ID.");
    }

    // Shorthand if (since no ternary operator)
    let is_of_age =  if age >= 21 {true} else {false};
    println!("Is of age: {}", is_of_age);
}