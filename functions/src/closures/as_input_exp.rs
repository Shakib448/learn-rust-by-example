/*
As input parameters

While Rust chooses how to capture variables on the fly mostly without  type annotation, this
ambiguity is not allowed when writing functions. When taking a closure as an input  parameter, the
closures's complete type must be annotated using one of a few traits, and they're determined by
what the closure dos with captured value.. In order of decreasing restriction, they are:

Fn: The closure uses the captured value by reference (&T)
FnMut: The closure uses the captured value by mutable reference (&mut T)
FnOnce: The closure uses the captured value by value (T)

On a variable-by-variable basis, the compiler will capture variables in the least restrictive manner possible

For instance, consider a parameter annotated as FnOnce. This specifies that the closure may capture
by &T, &mut T or T, but the compiler will ultimately chooses based on how the captured variables
are used in the closure.

This is because if a move is possible, then any type of borrow should also be possible. Note that the
reverse in not rue. If the parameter is annotated as Fn, then capturing variables by &mut T or T are
not allowed. However. &T is allowed

In the following example, try swapping the usage of FN, FnMut and FnOnce to see what happens
*/


fn apply<F>(f : F) where F: FnOnce() {
    f();
}

fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}


pub fn run() {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {greeting:?}");

        farewell.push_str("!!!");

        println!("Then I screamed {}", farewell);
        println!("Now I can sleep.");

        mem::drop(farewell);
    };
    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}