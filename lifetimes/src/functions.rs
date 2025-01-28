/*
 * Functions
 * Ignoring elision, function signatures with lifetimes have a few constraints:
 *      any reference must have an annotated lifetime.
 *      any reference being returned must have the same lifetimes as an input or be static.
 * Additionally, note that returning references without input is banned if it would result is returning
 * references to invalid data. The following example show off some valid form of functions with
 * lifetimes
 * */

fn print_one<'a>(x: &'a i32) {
    println!("`print one`: x is {}", x);
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, b: &'b i32) {
    println!("print multi: x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}
