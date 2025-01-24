/*
Phantom type parameters

A phantom type parameter is one that doesen't show up
at runtime, but is checked statically (and only) at compile time.

Data types can use extra generic type parameters to ac as markers or
to perform type checking at compile time. These extra parameters hold no storage values,
and have no runtime behavior.

In the following example, we combine std::marker::PhantomData with the phantom type parameter
concept to create tuples containing different data types.


*/


use std::marker::PhantomData;

#[derive(Debug)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(Debug)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

fn main() {
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomStruct<char, f32> = PhantomTuple("Q", PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
}

/*
Testcase: unit clarification

A useful method of unit conversions can be examined by
implementing Add with a phantom type parameter. The Add trait
is examined below:
*/


pub trait Add<RHS = Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

impl<U> Add for T<U> {
    type Output = T<U>;
}

use std::marker::PhantomData;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn run1() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);
}