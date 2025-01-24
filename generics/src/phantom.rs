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