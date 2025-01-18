/*
    Iterator::any

    Iterator::any is a function which when passed an iterator, will return true if any element satisfies
    the predicate. Otherwise, false. It's signature
*/

#![allow(dead_code)]

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
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    println!("Vec1 len : {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}