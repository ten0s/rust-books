use blog_oop::Post;

fn main() {
    let mut post = Post::new();
    assert_eq!("Draft", post.state());

    post.add_text("I ate a salad for lunch today");
    assert_eq!("Draft", post.state());
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("PendingReview: 0", post.state());
    assert_eq!("", post.content());

    post.add_text("?");
    assert_eq!("PendingReview: 0", post.state());
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("PendingReview: 1", post.state());
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("Draft", post.state());
    assert_eq!("", post.content());

    post.add_text("!!!");
    assert_eq!("Draft", post.state());
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("PendingReview: 0", post.state());
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("PendingReview: 1", post.state());
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Published", post.state());
    assert_eq!("I ate a salad for lunch today!!!", post.content());
}
