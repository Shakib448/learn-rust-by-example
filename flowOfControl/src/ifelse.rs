/*
Flow of Control
An integral part of any programming language are ways to modify control flow if else , for and other , let's talk about them in Rust
*/


/*
If/Else

Branching with if-else is similar to other languages. Unlike many of them, the boolean condition
doesn't need to be surrounded by parentheses, adn each condition is followed by a block. if-else
conditionals are expressions and all branches must return the same type.

*/

pub fn run () {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, increase ten-fold");
        n / 2
    };

    println!("{} -> {}", n, big_n);
}