use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //handle.join().unwrap();

    for i in 1..5 {
        println!("number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });
    handle.join().unwrap();
}
