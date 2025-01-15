
mod tuples_exp;
mod array_slices;
mod enum_exp;
mod pointers;
mod structs_exp;
pub fn run () {
    tuples_exp::run();
    array_slices::run();
    enum_exp::run();
    pointers::run();
    structs_exp::run();
}