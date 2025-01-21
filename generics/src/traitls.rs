/*
Traits

Of course trait s can also be generic. Here we define one which
reimplements the Drop trait as a generic method to drop itself and an input
*/

struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;
    empty.double_drop(null);
}