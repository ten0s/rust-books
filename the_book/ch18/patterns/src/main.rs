fn print_coord(&(x, y): &(i32, i32)) {
    println!("coord: ({}, {})", x, y);
}

/*
Function overloading is not supported.
Traits should be used instead
fn print_coord(&(x, y, z): &(i32, i32, i32)) {
    println!("coord: ({}, {}, {})", x, y, z);
}
*/

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "32".parse();

    if let Some(color) = favorite_color {
        println!("Use your favorite color: {}", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple color");
        } else {
            println!("Using orange color");
        }
    } else {
        println!("Using blue color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (_x, _y, _z) = (1, 2, 3);

    let point = (3, 5);
    print_coord(&point);

    /*
    Function overloading is not supported.
    Traits should be used instead
    let point = (3, 5, 7);
    print_coord(&point);
     */

    if let x = 5 {
        println!("Irrefutable pattern");
    }

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3..=5 => println!("three through five"),
        _ => println!("anything"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let x = 4;
    let y = false;
    match x {
        // behaves
        // like (4 | 5 | 6) if y        # wrong syntax
        // rather than 4 | 5 | (6 if y) # wrong syntax
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("x = {:?}, y = {:?}", x, y);

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("x = {:?}, y = {:?}", x, y);

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("quit");
        }
        Message::Move { x, y } => {
            println!("move to ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("write: {}", text);
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("change color to red: {}, green: {}, blue: {})", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "change color to hue: {}, saturation: {}, value: {})",
                h, s, v
            );
        }
    }

    let mut value = Some(5);
    let new_value = Some(10);
    match (value, new_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            value = new_value;
        }
    }
    println!("value is {:?}", value);

    let s = Some(String::from("Hello!"));
    /*
    Moves the value to _s
    if let Some(_s) = s {
        println!("found a string");
    }
     */
    // `_` doesn't bind at all
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("first: {}, last: {}", first, last);
        }
    }

    enum NewMessage {
        Hello { id: i32 },
    }
    let msg = NewMessage::Hello { id: 5 };
    match msg {
        NewMessage::Hello { id: id_var @ 3..=7 } => println!("Found an id in range: {}", id_var),
        NewMessage::Hello { id: 10..=12 } => println!("Found an id in another range"),
        NewMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}
