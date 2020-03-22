use std::sync::mpsc; // multiple producers single consumer
use std::thread;
use std::time::Duration;

fn one_message() {
    println!("one_message:");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //println!("val is {}", val); use of moved value error
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn many_messages() {
    println!("many_messages:");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
            String::from("four"),
            String::from("five"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn few_tx() {
    println!("few_tx:");
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
            String::from("four"),
            String::from("five"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("six"),
            String::from("seven"),
            String::from("eight"),
            String::from("niner"),
            String::from("ten"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    one_message();
    many_messages();
    few_tx();
}
