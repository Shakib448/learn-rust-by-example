/*
 *  Operator Overloading
 *
 *  In Rust, may of the operator can be overloaded via traits. That is, some operator can be use to
 *  accomplish different task based on their input arguments. This is possible because operators
 *  are syntactic sugar for method calls. For example, the + operator in a + b calls the add method
 *  (as in a.add(b)). This add method is part of the Add trait. Hence, the + operator can be use by
 *  any implementor of the Add trait.
 *
 *  A list  of the traits, such as Add, thtat overload operators can be found core::ops.
 *
 *
 * */

use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// By reversing the types, we end up implementing non-commutative addition.
// Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// This block implements the operation: Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
