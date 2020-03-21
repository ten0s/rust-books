use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let str1 = String::from("abcd");
    let str2 = "xyz";

    let res = longest(str1.as_str(), str2);
    println!("The longest string is {}", res);

    let novel = String::from("Call me Ishmael. Some year ago...");
    let first_sentence = novel.split('.').next().expect("Could not fa a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime";
}
