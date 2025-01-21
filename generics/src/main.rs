/*
Generics

Generics is the topic of generalizing types and functionalities
to border cases. This is extremely useful for reducing code
duplication in many ways, but can call for rather involved syntax. Namely
being generic requires taking great care to specify over which
types a generic type parameters.

A type parameter is specified as generic by the use of angle brackets and upper camel case;
<Aaa, Bbb, ...>. "Generic types parameters" are typically represented as <T>. In Rust, "generic" also
describes anything that accepts one or more generic type parameters <T>. Any type specified as a
generic type parameter is generic, and everything else is concrete (non-generic).

For example, defining a generic function named foo that takes an argument T of any type:
*/

fn foo<T>(arg: T) { ... }

/*
Because T has been specified as a generic type parameter using <T>, it is considered generic when
used here as (arg: T). This is the case even if T has previously been defined as a struct.

This example shows some of the syntax in action:
*/

struct A;

struct Single(A);

struct SingleGen<T>(T);


fn main() {
    let _s = Single(A);
    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}
