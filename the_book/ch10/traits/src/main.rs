use core::fmt::Display;
use std::fmt::Debug;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Sms {
    pub from: String,
    pub content: String,
}

impl Summary for Sms {
    fn summarize_author(&self) -> String {
        self.from.clone()
    }
}

fn notify(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    0
}

fn function2<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

fn returns_summarizable() -> impl Summary {
    Sms {
        from: String::from("someone"),
        content: String::from("some text"),
    }
}

/*
doesn't work
fn returns_summarizable2<T: Summary>() -> T {
    Sms {
        from: String::from("someone"),
        content: String::from("some text"),
    }
}
*/

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
    hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let sms = Sms {
        from: String::from("Billy"),
        content: String::from("this is some text"),
    };
    println!("1 new sms: {}", sms.summarize());
}
