fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice_fn_ptr(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn do_twice_fn_trait<F>(f: F, arg: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    // https://doc.rust-lang.org/nightly/std/primitive.fn.html

    // safe function pointer
    let ans1 = do_twice_fn_ptr(add_one, 5);
    println!("ans1 = {}", ans1);
    let ans1 = do_twice_fn_trait(add_one, 5);
    println!("ans1 = {}", ans1);

    // closure that doesn't capture its environment
    let ans2 = do_twice_fn_ptr(|x| x + 1, 5);
    println!("ans2 = {}", ans2);
    let ans2 = do_twice_fn_ptr(|x| x + 1, 5);
    println!("ans2 = {}", ans2);

    let one = 1;
    /*
    // close that captures its environment
    let ans3 = do_twice_fn_ptr(|x| x + one, 5);
                               ^^^^^^^^^^^ expected fn pointer, found closure
    */
    let ans3 = do_twice_fn_trait(|x| x + one, 5);
    println!("ans3 = {}", ans3);

    let numbers = vec![1, 2, 3];
    let strings: Vec<String> = numbers.iter().map(|i| i.to_string()).collect();
    // NB: `ToString` trait
    let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }
    // (0u32..20) is std::ops::Range
    let statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("statuses: {:?}", statuses);

    assert_eq!(11, returns_closure()(10));
}
