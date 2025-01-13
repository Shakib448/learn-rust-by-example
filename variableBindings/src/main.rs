/*
    Variable Bindings

    Rust provides type safety via static typing. Variable bindings can be type annotated when declared.
    However, in most cases, the compiler will be able to infer the type of variable from the context,
    heavily reducing the annotation burden.

    Values (like literals) can be bound to variables, using the let binding.
*/



fn main() {
    variable_fn();
}

fn variable_fn() {
    let an_integer = 1u32;
    let a_boolean = true;

    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;

     let noisy_unused_variable = 2u32;
}
