/*
Printing is handle by series of macros defined in std::fmt some of which are:
* format!: Write formatted text to String
* print!: Same as format! but the is printed to the console (io::stdout).
* println!: Same as print! but a newline is appended.
* eprint!: Same as print! but the text is printed to the standard error (io::stderr).
* eprintln!: Same as eprint! but a newline is appended.
All parse text the same fashion. As a plus, Rust check formatting correctness at the compile time.
*/

fn main() {
    println!("{} days", 31);

    //     The positional printing
    println!("{0}, this {1}. {1}, this is {0}", "Alice", "Bob");

    //     As can name printing
    println!("{subject} {verb} {object}", object = "The lazy dog", subject = "The Layz", verb = "The Verb");

    //     Different formatting
    println!("Base 10:                              {}",   69420);
    println!("Base 2 (binary):                      {:b}", 69420);
    println!("Base 8 (octal):                       {:o}", 69420);
    println!("Base 16 (hexadecimal):                {:x}", 69420);

    println!("{number:>5}", number=1); // 1
    println!("{number:0>5}", number=1); // 00001
    println!("{number:0<5}", number=1); // 10000

//     You can use named arguments in the format specifier by appending a "$"
    println!("{number:0>width$}", number=1, width=5);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    let number : f64 = 1.0;
    let width: usize  = 5;

    println!("{number:0>width$}");

}
