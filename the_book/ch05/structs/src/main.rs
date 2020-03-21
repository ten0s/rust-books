#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = build_user(String::from("user1@example.com"), String::from("user1"));
    println!("user1: {:?}", user1);

    let user2 = build_user(String::from("user2@example.com"), String::from("user2r"));
    println!("user2: {:?}", user2);

    user1.email = String::from("another@example.com");
    println!("user1: {:?}", user1);

    match_user(&user1);
    match_user(&user2);

    let user3 = User {
        username: String::from("user2"),
        ..user2
    };

    match_user(&user3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn match_user(user: &User) {
    match user {
        User { username, .. } if username == "user1" => println!("user1"),
        User { username, .. } if username == "user2" => println!("user2"),
        User { .. } => println!("user?"),
    }
}
