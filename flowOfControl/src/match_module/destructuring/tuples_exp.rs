
/*
    Destructuring
    A match block can destructure items in a variety of ways
    * Destructuring Tuples
    * Destructuring Arrays and Slices
    * Destructuring Enums
    * Destructuring Pointer
    * Destructuring Structures
*/

// Tuples
// Tuples can be destructured in a match as follows:

pub fn run() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("First is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
}