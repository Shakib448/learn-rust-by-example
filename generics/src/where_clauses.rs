/*
Where clauses

A bound can also be expressed using a where clause immediately before the
opening {, rather that at the type's first mention. Additionally, where clauses
can apply bounds to arbitrary types, rather that just to type parameters.

Some cases that a where clause is useful:
When specifying generic types and bounds separately is clearer:
*/

impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

impl<A, D> MyTrait<A, D> for YourType
where
    A: TraitB + TraitC,
    D: TraitE + TraitF,
{}


use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}