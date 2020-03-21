struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

trait MyIterator {
    fn my_next<T>(&mut self) -> Option<T>;
}

impl MyIterator for Counter {
    fn my_next<Counter>(&mut self) -> Option<Counter> {
        None
    }
}

impl MyIterator for String {
    fn my_next<String>(&mut self) -> Option<String> {
        None
    }
}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Beaver")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Supertraits

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

// Newtype Pattern

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct GenWrapper<T>(T);

use std::ops::Deref;

impl<T> Deref for GenWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for GenWrapper<Vec<String>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // can be pure self because GenWrapper<T> is Deref
        write!(f, "[{}]", self.0.join(", "))
    }
}

// TODO: How to make it work?
// TODO: GenWrapper should have hold kind of state...
impl Iterator for GenWrapper<String> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn main() {
    let it1 = Counter::new().my_next::<Counter>();
    let it2: Option<String> = String::new().my_next();

    let it3 = Counter::new().next();
    let it4 = String::new().chars().next();

    for n in Counter::new() {
        println!("n = {}", n);
    }

    for c in String::from("hello").chars() {
        println!("c = {}", c);
    }

    // using Newtype Pattern
    for c in GenWrapper(String::from("hello")) {
        println!("c = {}", c);
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(Millimeters(1000) + Meters(3), Millimeters(4000));

    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Supertraits

    let p = Point { x: 1, y: 3 };
    p.outline_print();

    // Newtype Pattern

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    let gw = GenWrapper(vec![String::from("hello"), String::from("world")]);
    println!("gw = {}", gw);
}
