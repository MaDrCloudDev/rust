// Vectors - Resizable arrays

// use std::mem; // can use this instead (like an import) and then just use "mem" in the example measuring the memory size of the vector

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; // DIFFERENCE BETWEEN VEC AND ARRAYS

    // Reassign a value (you can get numbers to mut, then reassign, however?)
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop(); // gets rid of the 6 from above

    // print
    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Print vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated (get the memory of the vector)
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values (similar to map in JS)
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers vec: {:?}", numbers);
}