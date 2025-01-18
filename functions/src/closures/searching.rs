/*
Searching through iterators

Iterator::find is a function which iterates over an iterator and searches for the first value which
satisfies some condition. If none of the values satisfy the condition, it returns None. It's signature:
*/

#![allow(unused)]

pub trait Iterator {
    type Item;

    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool;
}

pub fn run() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));


    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));
}
