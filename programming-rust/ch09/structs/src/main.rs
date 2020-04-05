pub mod extrema;
pub mod queue;
pub mod robot;

use std::fs::File;

use crate::extrema::Extrema;
use crate::queue::Queue;
use crate::robot::Robot;

fn main() {
    // Queue<char>
    let mut q = Queue::new();

    assert!(q.is_empty());

    q.push('0');
    assert!(!q.is_empty());
    println!("push('0') - {:?}", q);

    q.push('1');
    println!("push('1') - {:?}", q);

    assert_eq!(q.pop(), Some('0'));
    println!("pop() - {:?}", q);

    q.push('2');
    println!("push('2') - {:?}", q);
    assert_eq!(q.pop(), Some('1'));
    println!("pop() - {:?}", q);

    assert_eq!(q.pop(), Some('2'));
    println!("pop() - {:?}", q);

    assert!(q.is_empty());
    assert_eq!(q.pop(), None);

    // Queue<&'static str>
    let mut q = Queue::new();
    q.push("P");
    q.push("D");
    q.pop();
    q.push("X");
    let (front, back) = q.split();
    // q is now uninitialized
    assert_eq!(front, vec!["D"]);
    assert_eq!(back, vec!["X"]);

    // Extrema
    let a = [0, -3, 0, 15, 48];
    let e = Extrema::new(&a);
    assert_eq!(e.min(), -3);
    assert_eq!(e.max(), 48);

    // Robot (Interior Mutability)
    let log_file = File::create("robot.log").expect("open robot.log");
    let r = Robot::new(log_file);
    assert!(!r.has_hw_errors());
    // NB: r is non-mut!
    r.add_hw_error();
    assert!(r.has_hw_errors());
    // NB: r is non-mut
    r.log("hello log");
}
