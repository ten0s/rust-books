// Synonym for i32, not a separate type
// Not very usable this way
type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

// from std::io
type Result<T> = std::result::Result<T, std::io::Error>;

// empty type ! can be coerced into any other type
fn bar() -> ! {
    panic!();
}

fn foo() -> ! {
    // loop w/o break
    loop {
        println!("hi");
    }
}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 10;
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi!"));
    f()
}
