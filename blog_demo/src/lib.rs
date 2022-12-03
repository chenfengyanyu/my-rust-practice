pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

impl Post {
    pub fn new() -> Post {
        Post { state: Some(Box::new(Draft {})), content: String::new() }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    pub fn request_review(&mut self) {
        // Rust 不允许结构体中出现未被填充的值，因此通过 take 方法取出 state 字段的 Some 值，并在原来的位置留下一个 None
        if let Some(s) = self.state.take() {
            // request_review 方法会消耗当前的状态并返回一个新的状态
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&slef, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct  PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// 添加已发布状态
struct  Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&slef, post: &'a Post) -> &'a str {
        &post.content
    }
}