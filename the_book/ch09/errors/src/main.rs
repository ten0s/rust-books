use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = match File::open("file.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_v2() -> Result<String, io::Error> {
    // Result.? comes through From:from function
    let mut f = File::open("file.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("file.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string("file.txt")
}

use std::net::IpAddr;
fn parse_ip_addr() -> IpAddr {
    "127.0.0.1".parse().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = "hello.txt";
    let f = match File::open(file) {
        Ok(f1) => f1,
        Err(e1) => match e1.kind() {
            ErrorKind::NotFound => match File::create(file) {
                Ok(f2) => f2,
                Err(e2) => panic!("Creating file failed with: {:?}", e2),
            },
            _ => panic!("Opening file failed with: {:?}", e1),
        },
    };
    println!("{:?}", f);

    let g = File::open(file).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file).unwrap_or_else(|error| {
                panic!("Creating file failed with: {:?}", error);
            })
        } else {
            panic!("Opening file failed with: {:?}", error);
        }
    });
    println!("{:?}", g);

    let file2 = "not_existing.txt";
    // will panic with some default message
    //let h = File::open(file2).unwrap();
    // will panic with custom message
    //let h = File::open(file2).expect(&format!("Failed to open file: {}", file2));

    let f = File::open("some.txt")?;
    Ok(())
}
