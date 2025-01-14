use std::convert::{From, Into, TryFrom, TryInto};
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

#[derive(Debug)]'; ; ; ;
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