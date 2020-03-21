pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn state(&self) -> String {
        self.state.as_ref().unwrap().name()
    }

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_add_text() {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn name(&self) -> String;
    fn can_add_text(&self) -> bool {
        false
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

#[derive(Debug)]
struct Draft {}

impl State for Draft {
    fn name(&self) -> String {
        String::from("Draft")
    }

    fn can_add_text(&self) -> bool {
        true
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[derive(Debug)]
struct PendingReview {
    reviews: u8,
}

impl PendingReview {
    pub fn new() -> Self {
        Self { reviews: 0 }
    }

    pub fn reviews(&self) -> u8 {
        self.reviews
    }

    pub fn inc(&self) -> Self {
        Self {
            reviews: self.reviews + 1,
        }
    }
}

impl State for PendingReview {
    fn name(&self) -> String {
        format!("PendingReview: {}", self.reviews)
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.reviews() >= 1 {
            Box::new(Published {})
        } else {
            Box::new(PendingReview::inc(&self))
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

#[derive(Debug)]
struct Published {}

impl State for Published {
    fn name(&self) -> String {
        String::from("Published")
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
