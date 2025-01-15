/*
Binding
Indirectly accessing a variable makes it impossible to branch and use that variable without re-binding. match
Provides the @ sigil for binding values to names
*/

fn age() -> u32 {
    15
}
pub fn run() {
    match_binding();
    enum_binding();
}

fn match_binding() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
}

// You can also use binding to destructure enum variants, such as Option:

fn some_number() -> Option<u32> {
    Some(42)
}

fn enum_binding () {
    match some_number() {
        Some(n @ 42) => println!("The answer is {}", n),
        Some(n) => println!("Not the answer is {}", n),
        _ => (),
    }
}


