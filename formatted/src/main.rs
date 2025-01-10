use std::fmt::{self, Formatter, Display};

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
    println!("Base 10:                              {}", 69420);
    println!("Base 2 (binary):                      {:b}", 69420);
    println!("Base 8 (octal):                       {:o}", 69420);
    println!("Base 16 (hexadecimal):                {:x}", 69420);

    println!("{number:>5}", number = 1); // 1
    println!("{number:0>5}", number = 1); // 00001
    println!("{number:0<5}", number = 1); // 10000

    //     You can use named arguments in the format specifier by appending a "$"
    println!("{number:0>width$}", number = 1, width = 5);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    let number: f64 = 1.0;
    let width: usize = 5;

    println!("{number:0>width$}");
    dbg_fn();
    display_fn();
    testcase_fn();
    formatted_fn();
}

/*
    Debug
    All types which what to use std::fmt formatting tratis require an implementations are only provided for types
    such as in the std library. All others must be manually implemented somehow.

    The fmt::Debug tratis make this very straightforward. All types can derive (automatically create) the fmt::Debug implementation.
    This is not ture from for fmt::Display which must be manually implemented.
*/
#[allow(dead_code)]
struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);


#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);


#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}


fn dbg_fn() {
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christain", actor = "Christle");

    println!("Now {:?} will print!", Structure(0));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:#?}", peter);
}

/*
    fmt::Debug hardly looks compact and clean, so it is often advantageous to customize the output appearance. This is done by manually implementing
    fmt::Display, which uses the {} print marker. Implementing it looks like this:
*/
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn display_fn () {
    let minmax = MinMax(0,14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300,300);
    let small_range = MinMax(-3, 3);
    println!("The big range is {big} and the small is {small}",small=small_range,big=big_range);
    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}


/*
    Testcase:List
    Implementing fmt::Display for a structure where the elements must each be handled sequentially is tricky. The problem is that each
    write! generates a fmt::Result. Proper handling of this requires dealing with all the results. Rust provides the ? operator for exactly this purpose.

    Using ? on write! looks like this:
*/

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {

            if count != 0 {write!(f, " ")?;}
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }

}

fn testcase_fn() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}


/*
    We've seen that formatting is specified via a format string:
    format!("{}", foo) -> "3735928559"
    format!("0x{:X}", foo) -> "0xDEADBEEF"
    format!("0o{:o}", foo) -> "0o33653337357"

    The same variable (foo) can be formatted differently depending on which argument type is used: x vs o va unspecified.
    This formatting functionality is implemented via traits, and there is one trait for each argument type. The most common formatting
    trait is Display, which handles cases where the argument type is left unspecified: {} for instance.
*/


#[derive(Debug)]
struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lat >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    rea : u8,
    green: u8,
    blue: u8,
}

fn formatted_fn() {

    for city in [
        City {name : "Dublin", lat: 53.347778, lon: -6.259722 },
        City {name : "Oslo", lat: 59.95, lon: 10.75 },
        City {name : "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{:#?}", city);
    }

    for color in [
        Color { rea: 128, green: 255, blue: 90 },
        Color { rea: 128, green: 0, blue: 90 },

    ] {
        println!("{:#?}", color);
    }
}