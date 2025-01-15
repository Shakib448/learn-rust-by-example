/*
while let
Similar to if let while let can make awkward match sequences more tolerable. consider the
follow sequence that increments i:
*/

fn whilet_fn() {
    let mut optional = Some(7);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Got: {}", i);
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            }
            _ => { break; }
        }
    }
}

// Using while let makes this sequence much nicer:

fn fn_while() {
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

pub fn run() {
    whilet_fn();
    fn_while();
}