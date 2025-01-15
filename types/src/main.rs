// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]
/*

Types
Rust provides several mechanisms to change or define the type of primitive and user defined types.
The following sections cover:

Casting between primitive types
Specifying the desired type of literals
Using type inference
Aliasign types

*/


fn main() {
    println!("Hello, world!");
}

/*
   Casting
   Rust provides no implicit type conversion (coercion) between primitive types. But,  explicit type conversion (casting) can be performed using as keyword.

   Rules for converting between integral types follow C conventions generally, except in cases where C has undefined behavior.
   The behavior of all casts between integral types is well defined in Rust.
*/

fn casting_fn() {
    let decimal = 65.4321_f32;

    let integer: u8 = decimal;
    let integer = decimal as u8;
    let float = decimal as f32;

    let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, decimal);

    println!("1000 as a u16 is: {}", 1000 as u16);

    println!("1000 as a u8 is: {}", 1000 as u8);

    println!("-1 as a u8 is: {}", (-1i8) as u8);

    println!(" 128 a a i16 is: {}", 128 as i16);

    println!(" 128 as a i8 is: {}", 128 as i8);

    println!("1000 as a u8 is: {}", 1000 as u8);

    println!("232 as a i8 is: {}", 232 as i8);

    // 300.0 as u8 is 255
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("   nan as u8 is : {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:

    unsafe {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}


/*
    Literals

    Numeric literals can be type annotated by adding the type as a suffix. As an example, to specify that
    the literal 42 should have the type i32, write 42i32.

    The type of unsuffixed numeric literals will depend on how tye are used. If no constraint exists, the
    compiler will use i32 for integers, and f64 for floating-point numbers.

*/

fn literals_fn() {

    //     suffixed
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    //     unsuffixed
    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

/*
    The are some concepts used in the previous code that haven't been explained yet, here's a brief
    explanation for the impatient readers:
std::mem::size_of_val is a function, but called with its full path. Code can be split in logical units called modules.
In this case, the size_of_val function is defined in the mem module, and the mem module is defined in the std crate.
For more details, see modules and crates.

*/

/*
    Inference
    The type inference engine is pretty smart. It does more than looking at the type of the value expression during an initialization.
    It also looks at how the variable is used afterwards to infer its type. Here's an advanced example of type inference:
*/

fn inference_fn() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}

/*
Aliasing
The type statement can be used to give a logical name to an existing type. Types must have UpperCamelCase names, or the compiler will raise a warning.
 The exception to this rule are the primitive types: usize, f32, etc.
*/

fn aliasing_fn() {
    // `NanoSecond`, `Inch`, and `U64` are logical names for `u64`.
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;

    fn main() {
        // `NanoSecond` = `Inch` = `U64` = `u64`.
        let nanoseconds: NanoSecond = 5 as u64;
        let inches: Inch = 2 as U64;

        // Note that type aliases *don't* provide any extra type safety, because
        // aliases are *not* logical types
        println!("{} nanoseconds + {} inches = {} unit?",
                 nanoseconds,
                 inches,
                 nanoseconds + inches);
    }
}