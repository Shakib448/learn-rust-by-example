
mod match_exp;
mod destructuring;
mod guards;
pub fn run() {
    match_exp::run();
    destructuring::run();
    guards::run();
}