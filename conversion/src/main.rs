use std::convert::{From, Into, TryFrom, TryInto};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
/*
    Conversion
    Primitive type can  be converted to each other through casting.

    Rust addresses conversion between custom types (i.e sturct and enum) by the use of traits. The
    Generic conversions will be the From and Into traits. However, there are more specific ones for the
    more common cases, in particular when converting to and from String s.
*/

fn main() {
    println!("Hello, world!");
}


/*
    From and Into

    The from and Into trails are inherently linked, and this is actually part of its implementation. if you
    are able to convert type A from Type B, then it should be to believe that we should be able to
    convert type B to type A
*/

/*
    From
    The From trail allow for a type to define how to create itself from another type, hence providing a
    very simple mechanism for converting between several types. there are numerous implementations of this
    trait within the standard library for conversion of primitive and common types

    For example, we can easily convert as str into a String
*/

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Self { value: item }
    }
}

fn from_fn() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    let num = Number::from(32);
    println!("{:?}", nu\m);
}

/*
    The Into trait is simply the reciprocal of the From trait. It defines how to convert a type into another type.

    Calling into() typically requires us to specify the result type as the compiler is unable to determine the most of the time
*/
impl Into<Number> for i32 {
    fn into(self) -> Self {
        Self { value: self }
    }
}

fn into_fn() {
    let int = 5;
    let num: Number = int.into();
    println!("{:?}", num);
}

/*
From and Into are interchangeable

From and Into are designed to be complementary. We do not need to provide an implementation
for both traits. If you have implemented the From trait for your type, Into will call it when
necessary. Note, however, that the converse is not true: implementing Into for your type will not automatically provide it with an implementation of Form
*/


/*
    TryFrom and TryInto

    Similar to Form and Into, TryFrom and TryInto are generic trais for converting between type.
    Unlikke From/Into the TryFrom/Tryinto traits are used for fallible conversions, and as such return Result S.

*/



#[derive(Debug, PratialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value : i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn tryfrom_fn () {

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Ok(EvenNumber(5)));
}


/*
    To and form Strings

    Converting to String
    To convert any type to a String is as simple sas implementing the ToString trait for the type.
    Rather that doing so directly, you should implement the fmt::Display trait which automatically
    Provides ToString and also allows printing the types as discussed in the section on print!.
*/

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn strings_fn () {
    let circle = Circle { radius: 3 };
    println!("{}", circle.to_string());
}


/*
Parsing a String
It's useful to convert strings into many types, butt  one of the more common string operations is to
convert them form string to number. The idiomatic appraoch to this is to sue the parse function and either to arrange for
type inference or to specify the type to parse using hte "turbolfish" syntax.

this will convert the string into type specifed as long as the FromStr trait is implemented ofr that type. This is implemented for numerous types within the standard library.



*/

fn parsing_fn () {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {}", sum);
}


// The obtain this functionality on a user defined type simply implement the FromStr trait for the type

impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(x) => Ok(Circle { radius: x }),
            Err(e) => Err(e),
        }
    }
}


fn fn_prasing () {
    let radius = "     3 ";
    let circle : Circle = radius.parse().unwrap();

    println!("{:?}", circle);
}