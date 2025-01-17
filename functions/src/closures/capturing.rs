
/*
Capturing
Closure are inherently flexible and will do what the functionality requires to make teh closure work
without annotation. This allow capturing to flexibly adapt to the use case, sometimes moving and sometimes
borrowing. Closures can capture variables:

by reference : &T
by mutable reference: &mut T
by value : T

They preferentially capture variables by reference and only go lower when required.

*/