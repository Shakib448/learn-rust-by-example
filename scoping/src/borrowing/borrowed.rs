/*
Borrowing
Most of the time, we'd like to access data without taking ownership over it. To accomplish this, Rust
uses a borrowing mechanism. Instead of passing objects by value (T), objects can be passed by
reference (&T).

This compiler statically guarantees (via its borrow checker) that references always point to valid
objects. That is, while references to an object exist, the object cannot be destroyed.
*/

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is {}", borrowed_i32);
}

fn main() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;
        eat_box_i32(boxed_i32);

        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);
}