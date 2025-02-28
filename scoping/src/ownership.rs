/*
Ownership and moves

Because variables are in charge of freeing their own resources, resources can only have one
owner. this prevents resources from being freed more than once, Note that not all variables own
resources

When doing assignment (let x=y) or passing fuunction arguments by value(foo(x)), the
ownership of the resources is transferred, In Rust-speak, this is know as a move.

After moving resources, the previous owner can not loger be used. This avoiods creating
dangling Pointers

*/

// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
}

fn main() {
    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.

    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    //println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    //println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line
}

/*
Mutability
Mutability of data can be changed when ownership is transferred.

*/
fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;

    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}

/*
Partial moves
Withing the destructing of a single variable, both by-move and by-reference pattern
bindings can be used at the same time. Doing this will result ini a partial move of the
variable, which means that parts of the variable will be moved while others parts stay. In such
a case the parent variable cannot be used afterwards a a whole, however the parts that
are only reference (and not moved) can still be used.
*/

fn run1() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}

/*
In this example, wer store hte age variable on the heap illustrate the partial move: deleting ref in the
above code would give an error as the ownership of person.age would be move to the variable
age. if Person.age were stored on the stack, ref would not be required as the definition of age
would copy the data from person.age without moving it)
*/