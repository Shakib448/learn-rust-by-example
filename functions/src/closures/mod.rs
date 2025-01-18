mod index;
mod capturing;
mod as_input_exp;
mod type_anonoymity;
mod input_fn;
mod as_output;

pub fn run() {
    index::run();
    capturing::run();
    as_input_exp::run();
    type_anonoymity::run();
    input_fn::run();
    as_output::run();
}