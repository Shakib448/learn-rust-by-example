/*
Repeat
Macros can use + in the argument list to indicate that an argument may repeat at
least once, or *, to indicate that the argument may repeat zero or more
times.

In the following example, surrounding the matcher with $(...),+
will match one or more expression, separated by commas.
Also note that the semicolon is optional on the last case.


*/


macro_rules! find_win {
    ($x:expr) => ($x);

    ($x:expr, $($y:expr), +) => {
        std::cmp::min($x, find_win!($($y), +))
    }
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}