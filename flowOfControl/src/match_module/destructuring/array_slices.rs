/*
    arrays/slices

    Like tuples, arrays and slices can be destructured this way:
*/


pub fn run () {
    let array = [1,-2,6];

    match array {
        [0, second, third] => println!("array[0]= 0, array[1] = {}, array[2] = {}", second, third),
        [1, _, third] => println!("array[0] = 1, array[2] = {} and array[1] was ignored", third),
        [-1, second, ..] => println!("array[0] = -1, array[1] = {} and all the other ones were ignored", second),
        /*
            The code below would not compile
            [-1, second] => ...

            Or store them in another array/slices (the type depends on that of the value that is being matched against)
        */
        [3, second, tail @ ..] => println!("array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail),
        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}