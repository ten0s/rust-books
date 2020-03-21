// use std::collections::*;
// better
use std::collections::HashMap;

// use std::fmt;
// use std::io;
// or
use std::fmt::Result;
use std::io::Result as IoResult;

// use std::io
// use std::cmp::Ordering;
// better
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// better
// use std::io::{self, Write};

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

#[derive(Debug, PartialEq)]
enum PosInt {
    One,
    Two,
    Three,
}

fn main() {
    restaurant::eat_at_restaurant();
    // access possible due to `pub use::front_of_house::hosting` in lib.rs
    restaurant::hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, PosInt::One);
    map.insert(2, PosInt::Two);
    map.insert(3, PosInt::Three);
    println!("map: {:?}", map);

    assert_eq!(map.get(&1), Some(&PosInt::One));
    assert_eq!(map.get(&4), None);

    let mut list = std::collections::LinkedList::new();
    list.push_front(1);
    list.push_back(2);
    println!("list: {:?}", list);
}
