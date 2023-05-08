// Primitive strs = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // let hello = "Hello"; // imutable, fixed-length

    let mut growable_hello = String::from("Hello");

    // Get length
    println!("Length: {}", growable_hello.len());

    growable_hello.push('w'); // push method is for single chars or unicode

    growable_hello.push_str("orld"); // push_str method for pushing any length

    // Capacity in bytes
    println!("Capacity: {}", growable_hello.capacity());

    // Is empty
    println!("Is_Empty: {}", growable_hello.is_empty());

    // Contains
    println!("Contains_World: {}", growable_hello.contains("world"));

    // Replace
    println!("Replace: {}", growable_hello.replace("world", "earth"));

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing 
    assert_eq!(2, s.len()); // nothing happens because it passes
    assert_eq!(10, s.capacity());
    // assert_eq!(3, s.len()); // error - panics because assertion failed

    // Loop through string by whitespace
    for word in growable_hello.split_whitespace() {
        print!("{}", word);
    };

    // println!("{}", growable_hello)
}