#[allow(unused_variables)]
fn main() {
    let x = 5;
    let y = x;
    // x is still accessible because its size is known at compile time
    // Integer, Boolean, Float, Char, simple Tuples have
    // the Copy trait implemented
    // Integer, Boolean, Float, Char, simple Tuples don't have
    // the Drop trait implemented
    println!("x: {}, y: {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;
    // String doesn't have the Copy trait implemented
    //println!("s1: {}", s1); // error[E0382]: borrow of moved value: `s1`

    let s1 = String::from("hello");
    let s2 = s1.clone();
    // String has the Clone trait implemented
    // String has the Drop trait implemented
    println!("s1: {}, s2: {}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s);
    //println!("s: {}", s); // error[E0382]: borrow of moved value: `s`

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s3 = String::from("hello");
    let len = calc_len(&s1);
    println!("The len of {} is {}", s3, len);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}

fn calc_len(s: &String) -> usize {
    s.len()
}
