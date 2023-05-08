/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays (Arrays are of fixed length, Vectors are arrays that can grow)
*/

// Rust is a statically typed language, which means that it must know the types of all varibles at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 4543234673435;

    // Find max size usuable for these types
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;
    
    //  Get boolean from expression
    let is_greater: bool = 10 < 5;

    // Chars can only be one char or unicode (like emojis)
    let a1 = 'a';
    let face = '\u{1f600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}