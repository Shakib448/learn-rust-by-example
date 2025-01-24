/*
Scoping rules
Scopes play an important part in ownership, borrowing and lifetimes. That is,
they indicate to the compiler when borrows are valid, when resources can be freed
and when variables are created or destroyed
*/

/*
RAII

Variables in Rust do more than just hold data in the stack: they also own resources, e,g Box<T>
owns memory iin the heap. Rust enforces RAII(Resource Acquisition Is Initialization), so
whenever an object goes out of scope, it's destructor is called and its owned resources are freed.

This behavior shields against resource leak bugs, so you'll never have to manually free memory or worry
about memory leaks again! Here's quick showcase:
*/

// raii.rs
fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
}

fn main() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed
}

/*
Destructor
The notion of a destructor in Rust is provided through the Drop trait. The destructor is called
when the resource goes out of scope. This trait is not required to be implemented for every type, only
implement it for your type if you require its own destructor logic.

Run the below example to see how the drop trait works.When the variable in the main function goes out of
scope the custom destructor will e invoked
*/

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn run() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}