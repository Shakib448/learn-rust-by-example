#![allow(unused_variables)]
#![allow(dead_code)]

/*
    Variable Bindings

    Rust provides type safety via static typing. Variable bindings can be type annotated when declared.
    However, in most cases, the compiler will be able to infer the type of variable from the context,
    heavily reducing the annotation burden.

    Values (like literals) can be bound to variables, using the let binding.
*/


fn main() {
    variable_fn();
    scope_shadowing();
    scope_shadowing2();
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


/*
    Mutability

    Variable bindings are immutable by default, but this can be overridden using the mut modifier.
*/


fn mut_fn() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // _immutable_binding += 1;
}

/*
    Scope and Shadowing

    Variable bindings have a scope, and are constrained to live in a block. A block is a collection of statements enclosed by braces {}.
*/

fn scope_shadowing() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }

    println!("outer short: {}", long_lived_binding);
    println!("outer long: {}", long_lived_binding);
}

fn scope_shadowing2() {
    let shadowed_binding = 1;

    {
        println!("inner shadowing: {}", shadowed_binding);

        let shadowed_binding = "abc";
        println!("outer shadowing: {}", shadowed_binding);
    }

    println!("outer shadowing2: {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("inner shadowing: {}", shadowed_binding);

}