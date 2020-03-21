// $ cargo expand

// doesn't compile(
use macros::my_vec;

fn main() {
    let v: Vec<u32> = my_vec![1, 2, 3];
}
