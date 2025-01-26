/*
Explicit annotation
The borrow checker use explicit lifetime annotations to determine how long references should be valid.
In case where lifetimes are not elided , Rust requres explicit annotations to determine what
the lifetime of a reference should be. The syntax for explicitly annotation a lifetime uses an
apostrophe charter as follows

foo<'a>

Similar to closures, using lifetimes requires generics. Additionally this lifetime syntax indicates that the
lifetime of foo may not exceed that of 'a. Explicit annotation of a type has the form &'a T where 'a has already been introduced.

In case with multiple lifetimes, the syntax is similar:
foo<'a, 'b>
In this case, the lifetime of foo cannot exceed that of either 'a or 'b.

*/


fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("X is {} an y is {}", x, y)
}

fn failed_borrow<'a>() {
    let _x = 12;

    let _y: &'a i32 = &_x;
}


fn main() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);
    failed_borrow();
}
