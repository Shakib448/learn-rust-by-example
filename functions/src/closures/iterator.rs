/*
    Iterator::any

    Iterator::any is a function which when passed an iterator, will return true if any element satisfies
    the predicate. Otherwise, false. It's signature
*/

pub trait Iterator {
    type Item;

    fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool;
}


pub fn run () {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
}