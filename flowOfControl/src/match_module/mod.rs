
mod match_exp;
mod destructuring;
mod guards;
mod bindings;
pub fn run() {
    match_exp::run();
    destructuring::run();
    guards::run();
    bindings::run();
}