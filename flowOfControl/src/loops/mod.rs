
 mod loop_module;
 mod nesting_loop;
 mod returning_loop;
 mod whiles;

pub fn loops_mod () {
    loop_module::run();
    nesting_loop::run();
    returning_loop::run();
    whiles::run();
}