/*
 *  Drop
 *
 *  The Drop trait only has one method: drop, which is called automatically when an object goes out
 *  of scope. The main use of the Drop traits is to free the resources that implementor instance
 *  owns.
 *
 *  Box, Vec, String, File and Process are some examples of types that implement the Drop trait to
 *  fre resources. The drop trait can also be manually implemented for any custom data type.
 *
 *  The following example adds a print to console to the drop function to announce when it called.
 * */

struct Droppable {
    name: &'static str,
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variable can be manually dropped using the `drop` function
    drop(_a);
    // TODO ^ Try commenting this line

    println!("end of the main function");

    // `_a` *won't* be `drop`ed again here, because it already has been
    // (manually) `drop`ed
}
