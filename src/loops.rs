// Loops - used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break; // have to break, or it'll run forever
        }
    }

    // While loop (FizzBuzz: if divisibile by 5 and 15 fizzbuzz, by 3 fizz, by 5 buzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }
        // Increment
        count += 1
    }

    // For range
    for x in 0..50 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }

}