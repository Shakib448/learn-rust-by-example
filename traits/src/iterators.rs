/*
 * Iterators
 * The Iterators trait is used to implement iterators over collections such as arrays.
 * The trait requires only a method to be defined for the next element, which may be manually
 * defined in an impl block or automatically defined (as in arrays and ranges).
 *
 * As a point of convenience for common situations, the for construct turns some collections into
 * iterators using the .into_iter() method.
 *
 * */

struct Fibonacci {
    curr : u32,
    next : u32,
};


impl Iterators for Fibonacci {
    type Item = u32;


    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        
        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}


fn fibonacci() -> Fibonacci {
    Fibonacci { curr : 0, next: 1}
}

fn main() {
    // `0..3` is an `Iterator` that generates: 0, 1, and 2.
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` works through an `Iterator` until it returns `None`.
    // Each `Some` value is unwrapped and bound to a variable (here, `i`).
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
