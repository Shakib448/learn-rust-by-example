#![allow(unreachable_code, unused_labels)]

/*
    Nesting and labels

    It's possible to break or continue outer loops when dealing with nested loops. In these cases, the
    loops must be annotated with some 'label, and the label must be passed to the break / continue statement.
*/


pub fn run() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            break 'outer;
        }
        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}