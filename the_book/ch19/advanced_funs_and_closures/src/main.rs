fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let ans1 = do_twice(add_one, 5);
    println!("ans1 = {}", ans1);

    let ans2 = do_twice(|x| x + 1, 5);
    println!("ans2 = {}", ans2);

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
