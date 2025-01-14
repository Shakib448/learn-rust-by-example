
 mod loop_module;
 mod nesting_loop;
 mod returning_loop;
 mod whiles;
 mod for_range;

pub fn run_loops () {
    loop_module::run();
    nesting_loop::run();
    returning_loop::run();
    whiles::run();
    for_range::run();
}