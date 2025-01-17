/*
As input parameters

While Rust chooses how to capture variables on the fly mostly without  type annotation, this
ambiguity is not allowed when writing functions. When taking a closure as an input  parameter, the
closures's complete type must be annotated using one of a few traits, and they're determined by
what the closure dos with captured value.. In order of decreasing restriction, they are:

Fn: The closure uses the captured value by reference (&T)
FnMut: The closure uses the captured value by mutable reference (&mut T)
FnOnce: The closure uses the captured value by value (T)

*/