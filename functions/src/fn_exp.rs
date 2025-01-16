/*
Functions

Functions are declared using the fn keyword. it's argument are type annotated , just like variables,
and , if the function returns a value, hte return type must be specified after an arrow ->

The final expression in  the function will be used as return value. Alternatively the return
statement be used to return a value earlier from within the function, even from inside loops or if
statements

Let's rewrite FizzBuzz using functions!
*/

pub fn run () {
    fizzbuzz_to(100);
}


fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}