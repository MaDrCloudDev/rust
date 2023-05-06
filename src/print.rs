pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is from {}", "Matt", "Arizona");

    // Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Matt", "Arizona", "code"
    );

    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Matt",
        activity = "baseball"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait (tuple)
    println!("{:?}", (12, true, "Hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
