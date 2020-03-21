fn vec_usage() {
    let _v0: Vec<i32> = Vec::new();

    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("v1 = {:?}", v1);

    let mut v2 = vec![1, 2, 3];
    println!("v2 = {:?}", v2);
    for (i, n) in v2.iter().enumerate() {
        println!("i[{}]={}", i, n);
    }
    for n in &mut v2 {
        *n += 10;
    }
    println!("v2' = {:?}", v2);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row: {:?}", row);
}

fn string_usage() {
    let mut _s = String::new();

    let data = "initial contents";
    let _s = data.to_string(); // comes from the Display trait
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");

    let _hello = String::from("Hello");
    let _hello = String::from("Привет");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s = s1 + &s2;
    // s1 moved and not avaible any more
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    // both work since format! doesn't take ownership of its params
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    println!("{}", s1);
    let s = format!("{}-{}-{}", &s1, &s2, &s3);
    println!("{}", s);

    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    println!("{}", s);
    //let s = &hello[0..1]; crash due to wrong char boundary
    for c in hello.chars() {
        println!("{}", c);
    }
    for b in hello.bytes() {
        println!("{}", b);
    }
}

fn hashmap_usage() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    println!("{:?}", scores);

    let teams = vec![String::from("blue"), String::from("yellow")];
    let inits = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(inits.iter()).collect();
    println!("{:?}", scores);

    let key = String::from("key");
    let val = String::from("val");
    let mut map = HashMap::new();
    map.insert(key, val);
    // key and val have been moved into map
    //println!("{} {}", key, val);

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    let team = String::from("blue");
    let score = scores.get(&team);
    println!("{:?}", score);
    for (key, val) in &scores {
        println!("{}: {}", key, val);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 25);
    println!("{:?}", scores);
    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn main() {
    vec_usage();
    string_usage();
    hashmap_usage();
}
