/*

macro_rules!
Rust provides a powerful macro system that allows metaprogramming.
 As you've seen in previous chapters, macros look like functions,
 except that their name ends with a bang !, but instead of generating
 a function call, macros are expanded into source code that gets compiled with the rest of the program. However, unlike macros in C and other languages, Rust macros are expanded into abstract syntax trees, rather than string preprocessing, so you don't get unexpected precedence bugs.

Macros are created using the macro_rules! macro.

*/


macro_rules! say_hello {
    () => {
        println!("Hello!");
    }
}

fn main() {
    say_hello!();
    println!("Hello, world!");
}


/*
So why are macros useful?

Don't repeat yourself. There are many cases where you may need similar
functionality in multiple places but with different types. Often,
writing a macro is a useful way to avoid repeating code.
(More on this later)

Domain-specific languages. Macros allow you to define special syntax for
a specific purpose. (More on this later)

Variadic interfaces. Sometimes you want to define an interface that
takes a variable number of arguments. An example is println! which
could take any number of arguments, depending on the format string.
(More on this later)
*/
