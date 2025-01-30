/*
 *  Returning Traits with dyn
 *  The rust compiler needs to know how much psace every functions's return type requires. This
 *  means all you functions have to return a concrete type. Unlike other languages, if you have
 *  trait like Animal, you can't write a function that returns Animal, because its different
 *  implementations will need different amounts of memory.
 *
 * However, there's any easy workaround. Instead of returning a trait  object directly, our
 * functions return a Box which contains some Animal. A box is just reference to some memory in the
 * heap Because a reference has a statically-known size, and the compiler can guarantee it points
 * to a heap allocated Animal, we can return a trait from our function!
 *  
 *
 * */
