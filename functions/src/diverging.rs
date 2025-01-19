/*
Diverging functions

Diverging function never return. They are marked using !, which is an empty type

*/
#![feature(never_type)]


fn foo() -> ! {
    panic!("This call never returns");
}

/*
As opposed to all the other types, this one cannot be instantiated, because
the set of all possible values this type van have is empty. Note that,
it is different from the () type. Which has exactly one possible value.

For example, this function return as usual, although there is no
information  in the return value;
*/


fn some_fn() {
    ()
}

fn run1() {
    let _a = some_fn();
    println!("This function returns and you can see this line.")
}

/*
As opposed to this function, which will never return the control back to the caller.

*/

fn ran() {
    let x: ! = panic!("The call never returns");

    println!("You will never see this line!");
}


/*
Although this might seem like an abstract concept, it is actually very
often handy. The main advantage of this type is that it can be case
to any other type, maing it versatile in situations where an exact type
is required, such as in match branches. This felxibility allows us to write code
like this
*/

fn rua() {
    fn sum_add_numbers(up_to: u32) -> u32 {
        let mut acc = 0;

        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}