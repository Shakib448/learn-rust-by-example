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