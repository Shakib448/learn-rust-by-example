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