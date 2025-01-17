
/*
Capturing
Closure are inherently flexible and will do what the functionality requires to make teh closure work
without annotation. This allow capturing to flexibly adapt to the use case, sometimes moving and sometimes
borrowing. Closures can capture variables:

by reference : &T
by mutable reference: &mut T
by value : T

They preferentially capture variables by reference and only go lower when required.

*/

pub fn run () {
    use std::mem;

    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    print();

    let _reborrow = &color;
    print();
    let _color_moved = color;

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    let _count_reborrowed = &mut count;

    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume();
    // consume();
    move_fn();
}

// Using move before vertical pipes forces closure to take ownership of captured variables
fn move_fn () {
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&3));
}