use std::io::Write;
//use std::str::FromStr;

use gcd::gcd;

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        //numbers.push(u64::from_str(&arg).expect("error parsing argument"));
        numbers.push(
            arg.parse()
                .expect(format!("Parsing of `{}` failed", arg).as_str()),
        );
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The GCD of {:?} is {}", numbers, d);
}
