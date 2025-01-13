#![allow(dead_code)]

/*
    Custom Types

    Rust custom data types are formed mainly through the two keywords:

    struct: define a structure
    enum : define an enumeration

    Constants can also be created via the const and static keywords.
*/

fn main() {
    println!("Hello, world!");
}

/*
    Structures
    There are three types of structures ("structs") that can be created using the struct keyword :
    Tuples structs, which are, basically, named tuples.
    The classic C structs
    Unit structs, which are field-less, are useful for generics.
*/


#[derive(Debug)]

struct Person {
    name: String,
    age: u8,
}

struct Unit;
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn struct_fn() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 5.3, y: 5.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 10.3, ..another_point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}

/*
    Enums
    The enum keyword allows the creation of a type which may be one of a few different variants. Any
    Variant which is valid as a struct is also valid in an enum.
*/

enum WebEvent {
    PageLoad,
    PageUnload,
    // Like tuple structs
    KeyPress(char),
    Paste(String),
    // or c-like structs
    Click { x: i64, y: i64 },
}


fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        }
    }
}
fn enums() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}


/*
Type aliases
if you use a type alias, you can refer to each enum variant via it's alias. This might be useful if the
enum's name is too long or too generic, and you want to rename it
*/

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn type_fn() {
    let x = Operations::Add;
}

// The most common place you'll se the is in impl blocks using the Self alias.

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

/*
    The use declaration can be used so manual scoping isn't needed:

*/

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn use_fn() {
    use crate::Stage::{Beginner, Advanced};
    use crate::Role::*;

    let stage = Beginner;

    let role = Student;

    match stage {
        Beginner => println!("I am beginner"),
        Advanced => println!("I am advanced"),
    }

    match role {
        Student => println!("I am student"),
        Teacher => println!("I am teacher"),
    }
}

/*
    C-like
    enum can also be used as C-like enums.
*/

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
fn c_fn() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

/*
    Testcase: linked-list
    A common way to implement a linked-list is via enums:
*/

use crate::List::*;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: i32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail.
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn linked_fn () {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

/*
    constant
    Rust has two different types of constants which can be declared in any scope including global.
    Both require type annotation:

    const : An unchangeable value (the common case).
    static: A possibly mutable variable with 'static lifetime. The static lifetime is inferred and does not have to be specified.
    Accessing or modifying a mutable static variable is unsafe.

*/

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn cons_fn() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    THRESHOLD = 5
}