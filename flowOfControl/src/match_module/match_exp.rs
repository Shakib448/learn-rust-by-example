/*
match
Rust provides pattern matching via the match keyword, which can be used like a C switch. The first
matching arm is evaluated and al possible values must be covered.
*/
pub fn run() {
    let number = 13;

    println!("Tell me about {}", number);

    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}