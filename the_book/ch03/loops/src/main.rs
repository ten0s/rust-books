fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut i = 0;

    while i < a.len() {
        println!("a[{}]={}", i, a[i]);
        i += 1;
    }
    println!();

    i = a.len();
    while i > 0 {
        i -= 1;
        println!("a[{}]={}", i, a[i]);
    }
    println!();

    for e in a.iter() {
        println!("{}", e);
    }
    println!();

    for e in a.iter().rev() {
        println!("{}", e);
    }
    println!();

    for n in 1..5 {
        println!("{}", n);
    }
    println!();

    for n in (1..5).rev() {
        println!("{}", n);
    }
}
