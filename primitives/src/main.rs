/*
    Primitives

    Rust provides access to a wide variety of primitives. A simple includes:

    Scalar Types
    Singed integers: i8, i16, i32, i64, i128 and isize (pointer size)
    Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
    Floating point : f32, f64
    char Unicode scalar values like 'a' 'b' 'c' (4 bytes each)
    bool either true or false
    The unit type (), whose only possible value is an empty tuple: ()

    Despite the value of a unit type being a tuple, it is not considered
    a compound type because it does not contain multiple values.

    Compound Types
    Arrays like [1,2,3]
    Tuples (1, true)

    Variables can always be type annotated. Numbers may additionally be annotated
    via a suffix by default. Integers default to i32 and floats to f64. Note
    that Rust can also infer types from context.
*/

fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0;
    let default_integer = 7;
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;

    let mutable = true;
    mutable = true;


    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    let my_tuple = (5u32, 1u8, true, -5.04f32);
}


/*
    Literals and operators

    Integers 1, floats 1.2, characters 'a', strings "abc", booleans true and the unit type () can be expressed using literals.

    Integers can, alternatively, be expressed using hexadecimal, octal or binary notation using these prefixes respectively : 0x, 0o or 0b.

    Underscores can be inserted in numeric literals to improve readability, e.g. 1_000 is the same as 1000 and 0.000_001 is the same as 0.000001.

    Rust also supports scientific E-notation, e.g. 1e6, 7.6e-4. The associated type is f64.

    We need to tell the compiler the type of the literals we use. For now, we'll use the u32 suffix to indicate that the literal is an unsigned 32-bit
    integer, adn the i32 suffix to indicate that it's a signed 32-bit integer.

    The operators available and their precedence in Rust are similar to other C-like languages.
*/


fn literals_fn() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("not true is {}", !true);

    println!("0011 AND false is {}", true && false);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);
}

/*
    Tuples
    A tuple is a collection of values of different types. Tuples are constructed using parentheses (), and
    each tuple itself is a value with type signature (T1,T2, ...). where T1, T2 are the types of it's members.
    Functions can use tuples to return multiple values, as tuples can hols any number of values.
*/

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}


#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn tuple_fn() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);

    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}



/*
   Arrays and Slices

   An array is a collection of objects of the same type T, stored in contiguous memory. Arrays are created using brackets [], and their length,
   which is known at compile time, is part of their type signature [T; length].

   Slices are similar to arrays, but their length is not known at compile time. Instead, a slice is a two-world object; the first word is a pointer to the data,
   the second word is the length of the slice. The word size is the same as usize, determined by the processor architecture, e.g. 64 bits on an x86-64.
   Slices can be used to borrow a section of an array and have the type signature &[T].
*/


fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn slices_fn() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);
    println!("array size: {}", xs.len());
    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    println!("borrow the whole array as a slice");

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    let empty_array: [u32; 0] = [];

    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);


    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(x) => println!("The element at index {} is {}", i, x),
            None => println!("No element at index {}", i)
        }
    }
}