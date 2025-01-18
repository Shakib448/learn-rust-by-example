/*
Input functions

Since closures may be used as arguments, you might wonder if the same can be said about
functions. And indeed they can! if you declare a function that takes a closure as parameter, they any
function that satisfies the trait bound of that closure can be passed as a parameter.
*/

fn call_me<F>(f: F)
where
    F: Fn(),
{
    f();
}


fn function() {
    println!("I'm a function!");
}
pub fn run() {

    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(function);
}