/*
for loops

for and range
The for in construct can be use to iterate throuhg and Iterator. One of the esies wys to create an iterator is to use the range notation a..b. This yeilds
values from a (inclusive) to b (exlusive in steps of one

Let's write FirzzBuzz using for instead of while
*/


fn fizz_buzz() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }
}

/*
Alternatively, a..=b can be used for a range that is inclusive on both ends. The above can be written as:

*/

fn fizz_buzz2() {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }
}

/*
For and iterators
The for in construct is able interact with an iterator in several ways. As disscussed in the
Secion on the iterator trait, by default the for loop will apply the into_iter function to the
collection. However, this is not the only means of converting collections ito iterators.

into_iter, iter and iter_mut all handle the conversion of a collection into an iterator in different ways, by providing different views on the data within.
iter - This borrows each element of the collection through each iteration. this leaving the collection
untouched  and aviable for reuse after the loop
*/

fn iter_fn() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names : {:?}", names);
}

/*
into_iter - This consumes the collection so that on each iteration the exact data is provided.
Once the collection has been consumed it is no longer aviable for reuse as it has been 'moved' within the loop.
*/

fn into_iter_fn() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            // That is different
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // Into iter will not reuse
    // println!("names: {:?}", names);
}

// iter_mut - this mutably borrows each elemnt of the collection, allowing for the collection to
// be modifed in place

fn iter_mut_fn() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}
pub fn run() {
    fizz_buzz();
    fizz_buzz2();
    iter_fn();
    into_iter_fn();
    iter_mut_fn();
}