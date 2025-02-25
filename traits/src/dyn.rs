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
 * Rust tries to be as explicit as possible whenever it allocates memory on the head. So if you
 * function returns pointer-to-trait-on-heap in this way, you need to write the return type with
 * dyn keyword e.g Box<dyn Animal>
 *
 * */

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaaaaaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooooo!"
    }
}

fn radmon_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.243;
    let animal = radmon_animal(random_number);

    println!(
        "You've randomly chosen an animal, an it says {}",
        animal.noise()
    );
}
