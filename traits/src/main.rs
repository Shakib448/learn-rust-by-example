/*
 * Traits
 * A trait is a collection of methods defined for an unknown type: Self. They can access other
 * methods declared in the same trait.
 *
 * Traits can be implemented for any data type. In the example below, we define Animal, a group of
 * methods. The Animal trait is then implemented for the Sheep data type, allowing the use of
 * Methods from Animal with a Sheep.
 */

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn share(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaah?"
        } else {
            "baaaaaah!"
        }
    }

    fn talk(&self) {
        println!("{} pauses berefly.... {}", self.name, self.noise())
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.share();
    dolly.talk();
}
