/*
    pointers/ref

    For pointers, a distinction need to be mage to be between destructuring and dereferencing as they are different
    concepts which are use differently from languages like c/c++.

    Dereferencing uses *
    Destructuring uses &, ref and ref mut

*/

pub fn run() {
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;

    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}