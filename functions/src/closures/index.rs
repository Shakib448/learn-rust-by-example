/*
Closures

Closures are functions that can capture the enclosing environment. For example, a closure that
capture the x variable:

|val| val + x;

The syntax and capabilities of closures make them very convenient for on the fly usage. Calling closure
is exactly like calling a function. However, both input and return type can be inferred and input variable names must be specified

Other characteristics of closure include:

using || instead of () around input variables.
optional body delimitation ({}) for single line expression (mandatory otherwise)
the ability to capture the outer environment variables.


*/

pub fn run() {
    let outer_var = 42;

    // fn function(i: i32) -> i32 { i + outer_var }

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i: i32| i * outer_var;


    println!("closure_annotated() -> {}", closure_annotated(10));
    println!("closure_inferred() -> {}", closure_inferred(10));

    let one = || 1;

    println!("Closure returning one: {}", one());
    fun();
}

fn fun() {
    let mut stack: Vec<i32> = Vec::new();

    let mut push = |val: i32| {
        stack.push(val);
    };

    push(10);
    push(20);
    push(30);

    let mut peek = || -> Option<&i32> {
        stack.last()
    };

    if let Some(top) = peek() {
        println!("Current value is (peek): {}", top);
    };

    let mut pop = || -> Option<i32> {
        stack.pop()
    };

    if let Some(popped) = pop() {
        println!("Current value is (pop): {}", popped);
    };

    let len = || stack.len();

    println!("stack : {:?}", stack);
    println!("stack length : {:?}", len());
}

