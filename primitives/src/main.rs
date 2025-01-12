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




