/*
Syntax
In following subsections, we will show how to
define macros in Rust. There are three basic ideas:

Patterns and Designators
Overloading
Repetition
*/

/*
Designators

The arguments of a macro are prefixed by a dollar sign $ and type
annotated with a designator:
*/

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    }
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    }
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    println!({
        let x = 1u32;
        x * x + 2 * x - 1
    })
}


/*
These are some of the available designators:

block
expr is used for expressions
ident is used for variable/function names
item
literal is used for literal constants
pat (pattern)
path
stmt (statement)
tt (token tree)
ty (type)
vis (visibility qualifier)

*/