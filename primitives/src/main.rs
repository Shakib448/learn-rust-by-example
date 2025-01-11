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


fn main() {
    println!("Hello, world!");
}
