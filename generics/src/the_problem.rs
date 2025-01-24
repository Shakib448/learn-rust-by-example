/*
The Problem

A trait that is generic over it's container type has type
specification requirements - users of the trait must specify all
of it's generic types.

In the example below, the Contains trait allows the use of the
generic types A and B. The trait is then implemented for the
Container type, specifying i32 for A and B so that it can be used with
fn difference().

Because Contains is generic, we are forced to explicitly state all of the
generic types for fn difference(). In practice, we want a way tto express that
A and B are determined bby the input C. As you will see in the next section, associated
types provide exactly that capability.

*/

struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
             &number_1, &number_2, &container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is {}", difference(&container));
}

/*
Associated types
The use of "Associated types" improves the overall readability of code by
moving inner types locally into a trait as output types. Syntax for the
trait definition is a follows
*/

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}

fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{ ... }

fn difference<C: Contains>(container: &C) -> i32 { ... }


struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn run1() {
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);
    println!("Does container contain {} and {}: {}", &number_1, &number_2, &container.contains(&number_1, &number_2));

    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is {}", difference(&container));
}