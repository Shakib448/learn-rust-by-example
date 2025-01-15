/*
if let
For some use cases, when matching enums, match is awkward. For example:
*/

fn optional_fn () {
    let optional = Some(7);

    match optional {
        Some(i ) => println!("This is a really long string and `{:?}`", i),
        _ => {},
    }
}

// if let is cleaner for this use case and in addition allows various failure options to be specified:

fn fn_optional() {
    let number = Some(7);
    let letter : Option<i32> = None;
    let emotion: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emotion {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}

// In the same way, if let can be used to match any enum value:

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn iflet_fn () {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar  = a {
        println!("a is foobar");
    }

    if let Foo::Baz = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is foo {}", value);
    }
}

pub fn run () {
    optional_fn();
    fn_optional();
    iflet_fn();
}