
/*
Returning from loops

One of the uses of a loop is to retry an operation until it succeeds. If the operation returns a value
through, you might need to pass it to the rest of the code: put it after the break, and it will be
returned by the loop expression.
*/


pub fn run () {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}