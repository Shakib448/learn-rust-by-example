/*
Functions

The same set of rules can be applied to functions: a type T
becomes generic when preceded by <T>.

Using generic functions sometimes requires explicitly specifying type parameters.
This may be the case if the functions is called where the return type is generic,
or fi the compiler doesn't have enough information tto infer the necessary type parameters.

A function call with explicitly specified type parameters looks like : fun::<A, B, ...>().

*/

struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));
    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));
}