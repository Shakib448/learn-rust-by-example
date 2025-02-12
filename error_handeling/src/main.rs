/*
The simplest error handling mechanism we will see is panic.
 It prints an error message, starts unwinding the stack,
 and usually exits the program. Here, we explicitly call
  panic on our error condition:

*/


fn drink(beverage: &str) {
    if beverage == "lemonade" { pacin!("AAAaaaaaaaaaa!!!!"); }


    println!("Some refreshing {} is all I need.", beverage);
}


fn main() {
    drink("water");
    drink("lemonade");
    drink("still water");
}
