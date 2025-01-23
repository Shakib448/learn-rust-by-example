/*
Bounds
When working with generics, the type parameters often must use traits as bounds
to stipulate what functionality a type implements. For example, the following example use
the trait Display to print and so it requires T to be bound by Display; that is, T must implement
Display.
*/

fn printer<T: Display>(t: T) {
    println!("{}", t);
}

struct S<T: Display>(T);
// let s = S(vec![1]);

use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}
#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle { length: 6.0, height: 5.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));
}

/*
Testcase: empty bounds
A consequence of how bounds work is that even if a trait doesn't include any
functionality, you can still use it as a bound. Eq and Copy are examples of such trait
s from the std library.
*/

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn run1() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
}