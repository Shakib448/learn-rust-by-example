
/*
Type anonymity

Closures succinctly capture variables from enclosing scopes. Does this have any consequences? It surely does. Observe how using a closures
as a function parameter requires generics, which necessary because fo how they are defined:

*/

fn apply<F> (f: F) where F: FnOnce() {
    f();
}

/*
When a closure is defined, the compiler implicitly creates a new anonymous structure to store the captured variables inside,
meanwhile implementing the functionality via one of the traits: Fn, FnMut or FnOnce for this unknown type. This type is assigned
to the variable which is stored until calling.

Since this new is of unknown type, any usage in a function will require generics. However, an unbounded type parameter <T> would still be
ambiguous and not be allowed. Thus, bounding by one of the traits Fn, FnMut or FnOnce (which it implements) is sufficient to specify its type.
*/