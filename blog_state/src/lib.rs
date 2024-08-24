//! # blog_state
//!
//! `blog_state` is a crate that works for a post:
//!   create a draft, request a review and then publish it.
//!
//! It intends to show how to implement functionality using a
//! **state pattern** (object-oriented design pattern) 
//!

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, txt: &str) {
        self.content.push_str(txt);
    }

    pub fn content(&self) -> &str {
        match &self.state {
            Some(s) => s.content(self),
            None => "",
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
struct PendingReview {}
struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new() {
        let mut post = crate::Post::new();
        assert_eq!(post.content(), "");
        post.add_text("hello");
        assert_eq!(post.content(), "");
    }

    #[test]
    fn request_review() {
        let mut post = crate::Post::new();
        post.add_text("hello");
        post.request_review();
        assert_eq!(post.content(), "");
    }

    #[test]
    fn approve() {
        let mut post = crate::Post::new();
        post.add_text("hello");
        post.request_review();
        post.approve();
        assert_eq!(post.content(), "hello");
    }

    #[test]
    fn add_text() {
        let mut post = crate::Post::new();
        post.add_text("hello");
        assert_eq!(post.content, "hello");
    }

    #[test]
    fn content() {
        let mut post = crate::Post::new();
        post.add_text("hello");
        post.request_review();
        post.approve();
        assert_eq!(post.content(), "hello");
        post.state.take();
        assert_eq!(post.content(), "");
    }

    #[test]
    fn state_draft() {
        let mut post = crate::Post::new();
        post.add_text("hello");
        if let Some(mut s) = post.state.take() {
            assert_eq!(s.content(&post), "");
            s = s.approve();
            assert_eq!(s.content(&post), "");
            s = s.request_review();
            s = s.approve();
            assert_eq!(s.content(&post), "hello");
        }
    }

    #[test]
    fn state_pendingreview() {
        let mut post = crate::Post::new();
        post.add_text("hello");
        post.request_review();
        if let Some(mut s) = post.state.take() {
            assert_eq!(s.content(&post), "");
            s = s.request_review();
            assert_eq!(s.content(&post), "");
            s = s.approve();
            assert_eq!(s.content(&post), "hello");
        }
    }

    #[test]
    fn state_published() {
        let mut post = crate::Post::new();
        post.add_text("hello");
        post.request_review();
        post.approve();
        if let Some(mut s) = post.state.take() {
            assert_eq!(s.content(&post), "hello");
            s = s.request_review();
            assert_eq!(s.content(&post), "hello");
            s = s.approve();
            assert_eq!(s.content(&post), "hello");
        }
    }
}
